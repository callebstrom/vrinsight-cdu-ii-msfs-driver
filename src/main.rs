#![feature(strict_provenance, layout_for_ptr)]

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
                    Some(e) => {
                        log::trace!("Sending event {}", e);
                        msfs.send_event(e.clone());
                    }
                    None => {}
                }
            }
            Err(_) => {}
        }
    }
}
