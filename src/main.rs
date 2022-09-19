use std::time::Duration;

use serialport::{
    self, ClearBuffer, DataBits, FlowControl, Parity, SerialPort, SerialPortSettings, StopBits,
};

fn main() {
    get_device();
}

const RESET_COMMAND: &[u8] = &[
    0x43, 0x4d, 0x44, 0x52, 0x53, 0x54, 0x00, 0x4f, 0x43, 0x4d, 0x44, 0x43, 0x4f, 0x4e, 0x00, 0x4f,
]; // 0x434d44525354004f434d44434f4e004f; // CMDRST.OCMDCON.O".as_bytes();
const GET_FUNCTION_COMMAND: &[u8] = "CMDFUN.O".as_bytes();
const GET_VERSION_COMMAND: &[u8] = "CMDVER.O".as_bytes();

fn get_device() {
    let ports = serialport::available_ports().expect("Could not discover ports");

    let port = ports
        .into_iter()
        .find(|p| p.port_name == "COM7")
        .expect("No ports found!");

    connect_serial(&port);
}

fn connect_serial(port: &serialport::SerialPortInfo) -> () {
    let mut port = serialport::open_with_settings(
        &port.port_name,
        &SerialPortSettings {
            timeout: Duration::from_millis(1000),
            baud_rate: 9600,
            stop_bits: StopBits::One,
            data_bits: DataBits::Seven,
            parity: Parity::Even,
            ..Default::default()
        },
    )
    .expect("Could not connect to serial port");

    println!("Connected");
    configure(&mut *port);

    // test_single_port(&mut *port, true);

    loop {
        let mut buf = [0u8; 12];
        let raw = port.read_exact(&mut buf);
        match raw {
            Ok(_) => println!("{}", buf[0]),
            Err(err) => println!("Error: {}", err.to_string()),
        }
    }
}

fn configure(port: &mut dyn serialport::SerialPort) -> () {
    println!("Configuring");
    port.write(RESET_COMMAND).expect("Could not reset device");
    let mut buf = [0u8; 8];
    let raw = port.read_exact(&mut buf);
    match raw {
        Ok(_) => println!("{}", buf[0]),
        Err(err) => println!("Error: {}", err.to_string()),
    }
    println!("Reset device");
    port.write(GET_FUNCTION_COMMAND)
        .expect("Could not get function from device");
    println!("Got version");
    port.write(GET_VERSION_COMMAND)
        .expect("Could not get version from device");
}
