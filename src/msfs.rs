use std::{borrow::BorrowMut, ffi::c_void, thread::sleep, time::Duration};

use rand::Rng;
use simconnect::{
    DWORD, SIMCONNECT_EVENT_FLAG_GROUPID_IS_PRIORITY, SIMCONNECT_GROUP_PRIORITY_HIGHEST,
    SIMCONNECT_OBJECT_ID_USER,
};

const MOBIFLIGHT_PREFIX: &str = "MobiFlight";
const ATC_MODEL_DEFINITION_ID: DWORD = 0;

pub struct MSFS {
    connection: simconnect::SimConnector,
}

impl MSFS {
    pub fn new(app_name: &str) -> MSFS {
        let mut connection = simconnect::SimConnector::new();

        Self::connect(connection.borrow_mut(), app_name);

        connection.add_data_definition(
            ATC_MODEL_DEFINITION_ID,
            "ATC MODEL",
            "",
            simconnect::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_STRING32,
            0,
        ); // Assign a sim variable to a client defined id

        return MSFS { connection };
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
                _ => {
                    // log::warn!("Received unexpected response in open call: {:#?}", res);
                }
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
                                    let data = simobject_data.dwData;

                                    let p_data = &simobject_data;

                                    let start = data as *mut c_void;

                                    let mut result: Vec<i8> = vec![0; 128];
                                    let result_start = result.as_mut_ptr();
                                    let mut result_size: u32 = 0;

                                    let cb_data = std::mem::size_of_val(&simobject_data) as u32;

                                    simconnect::SimConnect_RetrieveString(
                                        std::mem::transmute(p_data),
                                        cb_data,
                                        start,
                                        std::ptr::addr_of!(result_start).cast_mut(),
                                        std::ptr::addr_of_mut!(result_size),
                                    );

                                    log::trace!("ATC_MODEL result size {:#?} bytes", result_size);
                                    break;
                                }
                            }
                            _ => log::warn!("Unknown defineID received"),
                        }
                    },
                    Ok(simconnect::DispatchResult::Exception(e)) => unsafe {
                        let error_ptr = e.dwException;
                        let error = std::ptr::from_exposed_addr::<u8>(error_ptr as usize);
                        log::error!(
                            "Could not determine aircraft type due to SimConnect exception: {:#?}",
                            *error
                        );
                    },
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
        }

        return String::from("C25C");
    }

    pub fn send_event(&mut self, event: &String) {
        let event_id = 0;
        let group_id = 1;
        let event_name = format!("{MOBIFLIGHT_PREFIX}.{event}");
        if !self
            .connection
            .map_client_event_to_sim_event(event_id, event_name.as_str())
        {
            panic!("Could not register event");
        }

        if !self
            .connection
            .add_client_event_to_notification_group(group_id, event_id, false)
        {
            panic!("Could not add client event");
        }

        if !self
            .connection
            .set_notification_group_priority(group_id, SIMCONNECT_GROUP_PRIORITY_HIGHEST)
        {
            panic!("Could not add client event");
        }

        if !self.connection.transmit_client_event(
            SIMCONNECT_OBJECT_ID_USER,
            event_id,
            0,
            group_id,
            SIMCONNECT_EVENT_FLAG_GROUPID_IS_PRIORITY,
        ) {
            panic!("Could not transmit event");
        }
    }
}
