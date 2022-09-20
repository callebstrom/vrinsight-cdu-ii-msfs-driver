mod cdu;
mod msfs;

use cdu::CDU;
use msfs::MSFS;

fn main() {
    let mut cdu = cdu::CDU::new();
    let msfs = msfs::MSFS::new("VRInsight CDU II MSFS Driver");
    loop {
        match cdu.read() {
            Ok(message) => println!("{}", message),
            Err(error) => println!("{}", error),
        }
    }
}
