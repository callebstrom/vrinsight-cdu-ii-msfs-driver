use std::{thread::sleep, time::Duration};

use simconnect::{
    SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_STRING128,
    SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_STRING32,
    SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_STRINGV, SIMCONNECT_EVENT_FLAG_GROUPID_IS_PRIORITY,
    SIMCONNECT_GROUP_PRIORITY_HIGHEST, SIMCONNECT_OBJECT_ID_USER,
    SIMCONNECT_PERIOD_SIMCONNECT_PERIOD_ONCE, SIMCONNECT_PERIOD_SIMCONNECT_PERIOD_SIM_FRAME,
    SIMCONNECT_RECV_SIMOBJECT_DATA,
};

const MOBIFLIGHT_PREFIX: &str = "MobiFlight";

pub struct MSFS<'a> {
    app_name: &'a str,
}

#[derive(Debug)]
struct DataStruct {
    model: [char; 32],
}

impl MSFS<'_> {
    pub fn new(app_name: &str) -> MSFS {
        return MSFS { app_name };
    }

    pub fn send_event(&self, event: &String) {
        let mut conn = simconnect::SimConnector::new();

        if !conn.connect(self.app_name) {
            panic!("Could not connect via SimConnect. Is simulator running?");
        }

        conn.add_data_definition(
            0,
            "ATC MODEL",
            "",
            simconnect::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_STRING32,
            u32::MAX,
        ); // Assign a sim variable to a client defined id
        conn.request_data_on_sim_object(
            0,
            0,
            0,
            simconnect::SIMCONNECT_PERIOD_SIMCONNECT_PERIOD_SIM_FRAME,
            0,
            0,
            0,
            0,
        );

        loop {
            match conn.get_next_message() {
                Ok(simconnect::DispatchResult::SimobjectData(data)) => unsafe {
                    match data.dwDefineID {
                        0 => {
                            // let dw_data = data.dwData;
                            // let data = dw_data as *const DataStruct;

                            // let a: DataStruct = std::mem::transmute_copy(&data);
                            // println!("{:#?}", a);
                            break;
                        }
                        _ => (),
                    }
                },
                _ => (),
            }

            sleep(Duration::from_millis(16)); // Will use up lots of CPU if this is not included, as get_next_message() is non-blocking
        }
        let event_id = 0;
        let group_id = 1;
        let event_name = format!("{MOBIFLIGHT_PREFIX}.{event}");
        if !conn.map_client_event_to_sim_event(event_id, event_name.as_str()) {
            panic!("Could not register event");
        }

        if !conn.add_client_event_to_notification_group(group_id, event_id, false) {
            panic!("Could not add client event");
        }

        if !conn.set_notification_group_priority(group_id, SIMCONNECT_GROUP_PRIORITY_HIGHEST) {
            panic!("Could not add client event");
        }

        if !conn.transmit_client_event(
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
