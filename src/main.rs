mod cdu;
mod keymap;
mod msfs;

fn main() {
    let keymap = keymap::KeyMap::new();
    let mut cdu = cdu::CDU::new();
    let msfs = msfs::MSFS::new("VRInsight CDU II MSFS Driver");

    loop {
        match cdu.read() {
            Ok(message) => {
                let event = keymap.get_event(&message);
                msfs.send_event(&event);
            }
            Err(_) => {}
        }
    }
}
