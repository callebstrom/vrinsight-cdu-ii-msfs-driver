#[macro_use]
extern crate serde;

use std::{time::{Instant, self}, thread};

mod cdu;
mod keymap;
mod msfs;

const KEEP_ALIVE_INTERVAL: u128 = 5000;
                        
fn process_event(event: &keymap::Action, msfs: &mut msfs::MSFS) {
    match event {
        keymap::Action::EventWithoutValue(eventWithoutValue) => {
            msfs.send_event(eventWithoutValue.clone());
        }
        keymap::Action::EventWithValue(eventWithValue) => {
            msfs.send_event_with_value(eventWithValue.event.clone(), eventWithValue.value);
        }
        keymap::Action::Delay(delay) => {
            thread::sleep(time::Duration::from_millis(delay.delay));
        }
        keymap::Action::VarWithValue(var) => {
            msfs.set_var(var.var.clone(), var.value);
        }
    }
}

fn main() {
    simple_logger::SimpleLogger::new().env().init().unwrap();

    let keymap = keymap::KeyMap::new();
    let mut cdu = cdu::CDU::new(keymap.get_port());
    let mut msfs = msfs::MSFS::new("VRInsight CDU II MSFS Driver");

    let mut last_keep_alive = Instant::now();

    loop {
        match cdu.read() {
            Ok(message) => {
                log::trace!("Received key: {}", &message);

                let aircraft_icao = msfs.determine_aircraft_type();
                log::trace!("Received aircraft key: {}", aircraft_icao);

                match keymap.get_event(&aircraft_icao, &message) {
                    Some(eventOrSequence) => match eventOrSequence {
                        keymap::ActionOrActionSequence::Single(action) => {
                            process_event(action, &mut msfs);
                        },
                        keymap::ActionOrActionSequence::Sequence(sequence) => {
                            sequence.iter().for_each(|action| process_event(action, &mut msfs));
                        },
                        
                    },
                    None => {}
                }

                cdu.keep_alive();
            }
            Err(_) => {}
        }

        let elapsed_since_keep_alive = Instant::now() - last_keep_alive;

        if elapsed_since_keep_alive.as_millis() >= KEEP_ALIVE_INTERVAL {
            cdu.keep_alive();
            last_keep_alive = Instant::now();
        }
    }
}
