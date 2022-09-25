#![feature(strict_provenance, layout_for_ptr)]

#[macro_use]
extern crate serde;

mod cdu;
mod keymap;
mod msfs;

fn main() {
    simple_logger::SimpleLogger::new().env().init().unwrap();

    let keymap = keymap::KeyMap::new();
    let mut cdu = cdu::CDU::new();
    let mut msfs = msfs::MSFS::new("VRInsight CDU II MSFS Driver");

    loop {
        match cdu.read() {
            Ok(message) => {
                log::trace!("Received key: {}", &message);
                let aircraft_icao = msfs.determine_aircraft_type();
                let event = keymap.get_event(&aircraft_icao, &message);
                match event {
                    Some(e) => match e {
                        keymap::Event::WithoutValue(event) => {
                            msfs.send_event(event.clone());
                        }
                        keymap::Event::WithValue { event, value } => {
                            msfs.send_event_with_value((*event).clone(), *value);
                        }
                    },
                    None => {}
                }
            }
            Err(_) => {}
        }
    }
}
