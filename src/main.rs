mod cdu;
mod keymap;
mod msfs;

fn main() {
    simple_logger::SimpleLogger::new().env().init().unwrap();

    let keymap = keymap::KeyMap::new();
    let mut cdu = cdu::CDU::new();
    let msfs = msfs::MSFS::new("VRInsight CDU II MSFS Driver");

    loop {
        match cdu.read() {
            Ok(message) => {
                // ATC MODEL
                let event = keymap.get_event(&"C25C".to_string(), &message);
                match event {
                    Some(e) => {
                        log::trace!("Sending event {}", e);
                        msfs.send_event(&e);
                    }
                    None => {}
                }
            }
            Err(_) => {}
        }
    }
}
