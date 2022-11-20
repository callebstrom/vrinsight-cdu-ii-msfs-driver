use regex::Regex;
use std::{borrow::BorrowMut, collections::HashMap, time::Duration};

use rand::Rng;
use simconnect::{
    DWORD, SIMCONNECT_EVENT_FLAG_GROUPID_IS_PRIORITY, SIMCONNECT_GROUP_PRIORITY_HIGHEST,
    SIMCONNECT_OBJECT_ID_USER,
};

const MOBIFLIGHT_PREFIX: &str = "MobiFlight";
const ATC_MODEL_DEFINITION_ID: DWORD = 0;
const GROUP_ID: DWORD = 1;

pub struct MSFS {
    connection: simconnect::SimConnector,
    event_map: HashMap<String, DWORD>,
    current_event_id: DWORD,
}

struct AtcModelResult {
    atc_model: [u8; 64],
}

impl MSFS {
    pub fn new(app_name: &str) -> MSFS {
        let mut connection = simconnect::SimConnector::new();

        Self::connect(connection.borrow_mut(), app_name);

        connection.add_data_definition(
            ATC_MODEL_DEFINITION_ID,
            "ATC MODEL",
            "",
            simconnect::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_STRING64,
            0,
        ); // Assign a sim variable to a client defined id

        return MSFS {
            connection,
            event_map: HashMap::new(),
            current_event_id: 0,
        };
    }

    fn connect(connection: &mut simconnect::SimConnector, app_name: &str) {
        if !connection.connect(app_name) {
            log::warn!("Could not connect via SimConnect. Is simulator running?");
            return;
        }

        std::thread::sleep(Duration::from_millis(16));

        loop {
            match connection.get_next_message() {
                Ok(simconnect::DispatchResult::Open(open)) => {
                    let app_major = open.dwApplicationBuildMajor;
                    let app_minor = open.dwApplicationBuildMinor;
                    let sim_connect_major = open.dwSimConnectBuildMajor;
                    let sim_connect_minor = open.dwSimConnectBuildMinor;
                    log::trace!(
                        "Successfully opened connection to simulator version {}.{} and SimConnect version {}.{}",
                        app_major,
                        app_minor,
                        sim_connect_major,
                        sim_connect_minor
                    );
                    break;
                }
                _ => {}
            }
            std::thread::sleep(Duration::from_millis(16));
        }
    }

    pub fn determine_aircraft_type(&self) -> String {
        let request_id: DWORD = rand::thread_rng().gen();

        if self.connection.request_data_on_sim_object(
            request_id,
            ATC_MODEL_DEFINITION_ID,
            SIMCONNECT_OBJECT_ID_USER,
            simconnect::SIMCONNECT_PERIOD_SIMCONNECT_PERIOD_ONCE,
            0,
            0,
            0,
            0,
        ) {
            loop {
                match self.connection.get_next_message() {
                    Ok(simconnect::DispatchResult::SimobjectData(simobject_data)) => unsafe {
                        match simobject_data.dwDefineID {
                            ATC_MODEL_DEFINITION_ID => {
                                if simobject_data.dwRequestID == request_id {
                                    let response = &simobject_data;
                                    let dw_data = std::ptr::addr_of!(response.dwData)
                                        as *const AtcModelResult;

                                    let atc_model = Self::parse_atc_model(&*dw_data)
                                        .unwrap_or("Unknown".to_string());

                                    log::trace!("Received ATC_MODEL {}", atc_model);

                                    return atc_model.trim().to_string().replace("\0", "");
                                }
                            }
                            _ => log::warn!("Unknown defineID received"),
                        }
                    },
                    Ok(simconnect::DispatchResult::Exception(..)) => {
                        // Data is not yet ready
                    }
                    Ok(simconnect::DispatchResult::Null) => {
                        log::error!("Could not determine aircraft type: null");
                    }
                    Err(e) => {
                        log::error!("Could not determine aircraft type: {}", e);
                    }
                    _ => {}
                }

                std::thread::sleep(Duration::from_millis(16));
            }
        } else {
            log::error!("Could not determine aircraft type");
            return "".to_string();
        }
    }

    fn register_event(&mut self, event_name: String) -> DWORD {
        log::trace!("Registering event \"{}\"", event_name);

        self.current_event_id += 1;
        let next_event_id = self.current_event_id;

        if self
            .connection
            .map_client_event_to_sim_event(next_event_id, event_name.as_str())
        {
            self.event_map.insert(event_name.to_string(), next_event_id);
        } else {
            log::error!("Could not register event");
        }

        if !self
            .connection
            .add_client_event_to_notification_group(GROUP_ID, next_event_id, false)
        {
            log::error!("Could not add client event");
        }

        if !self
            .connection
            .set_notification_group_priority(GROUP_ID, SIMCONNECT_GROUP_PRIORITY_HIGHEST)
        {
            log::error!("Could not add client event");
        };

        return next_event_id;
    }

    pub fn send_event(&mut self, event: String) {
        self.send_event_with_value(event, 0);
    }

    pub fn send_event_with_value(&mut self, event: String, value: DWORD) {
        let is_html_event = &event.starts_with("H:");
        let event_name = if *is_html_event {
            let event_without_prefix = event.split_at(2).1;
            format!("{MOBIFLIGHT_PREFIX}.{event_without_prefix}")
        } else {
            event.clone()
        };
        
        log::trace!("Sending event \"{}\" with value \"{}\"", event_name, value);

        let event_id: DWORD = if !self.event_map.contains_key(&event_name) {
            self.register_event(event_name)
        } else {
            *self
                .event_map
                .get(&event_name)
                .expect("event_map changed after container_key check")
        };

        if !self.connection.transmit_client_event(
            SIMCONNECT_OBJECT_ID_USER,
            event_id,
            value,
            GROUP_ID,
            SIMCONNECT_EVENT_FLAG_GROUPID_IS_PRIORITY,
        ) {
            panic!("Could not transmit event");
        }
    }

    fn parse_atc_model(dw_data: &AtcModelResult) -> Option<String> {
        let atc_model_raw = std::str::from_utf8(&(*dw_data).atc_model).unwrap();
        log::trace!("Parsing raw ATC MODEL: {}", atc_model_raw);
        let ac_model_regex = Regex::new(r".*AC_MODEL[_]{0,1}(.*)\.0").unwrap();
        ac_model_regex
            .captures(atc_model_raw)
            .map(|matches| matches.get(1).unwrap())
            .map(|first_match| first_match.as_str().to_string())
            .or_else(|| Some(atc_model_raw.trim().to_string()))
    }
}
