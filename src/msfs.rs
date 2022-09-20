use simconnect::{
    SIMCONNECT_EVENT_FLAG_GROUPID_IS_PRIORITY, SIMCONNECT_GROUP_PRIORITY_HIGHEST,
    SIMCONNECT_OBJECT_ID_USER,
};

pub struct MSFS<'a> {
    app_name: &'a str,
}

impl MSFS<'_> {
    pub fn new(app_name: &str) -> MSFS {
        return MSFS { app_name };
    }

    pub fn send_event(&self, event: &String) {
        let event_id = 0;
        let group_id = 1;
        let event_name = format!("MobiFlight.{event}");

        let mut conn = simconnect::SimConnector::new();

        if !conn.connect(self.app_name) {
            panic!("Could not connect via SimConnect. Is simulator running?");
        }
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
