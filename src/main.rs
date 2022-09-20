use std::time::Duration;

use serialport::{self, DataBits, FlowControl, Parity, SerialPortSettings, StopBits};

fn main() {
    get_device();
}

const RESET_COMMAND: &[u8] = &[0x43, 0x4d, 0x44, 0x52, 0x53, 0x54, 0x00, 0x00]; // CMDRST..
const CONNECT_COMMAND: &[u8] = &[0x43, 0x4d, 0x44, 0x43, 0x4f, 0x4e, 0x00, 0x00]; // CMDCON..
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
            timeout: Duration::from_millis(250),
            baud_rate: 115200,
            stop_bits: StopBits::One,
            data_bits: DataBits::Eight,
            flow_control: FlowControl::None,
            parity: Parity::None,
        },
    )
    .expect("Could not connect to serial port");

    println!("Connected");
    configure(&mut *port);

    loop {
        let mut buf: Vec<u8> = vec![0; 8];
        let raw = port.read_exact(buf.as_mut());
        let res = String::from_utf8(buf).expect("yes");
        match raw {
            Ok(_) => println!("{}", res),
            Err(_err) => {}
        }
    }
}

fn get_command_response(port: &mut dyn serialport::SerialPort) -> Result<Vec<u8>, std::io::Error> {
    let mut buf: Vec<u8> = vec![0; 8];
    return port.read_exact(buf.as_mut()).map(|_| buf);
}

fn configure(port: &mut dyn serialport::SerialPort) -> () {
    println!("Configuring");

    port.write(RESET_COMMAND).expect("Could not reset device");
    std::thread::sleep(Duration::from_secs(5));

    port.write(CONNECT_COMMAND)
        .expect("Could not connect to device");
    std::thread::sleep(Duration::from_secs(5));

    get_command_response(port).expect("Could not reset device");

    println!("Reset device");

    port.write(GET_FUNCTION_COMMAND)
        .expect("Could not get device type");

    let device_type = get_command_response(port).expect("Could not reset device");

    println!(
        "Device type: {}",
        String::from_utf8(device_type)
            .unwrap_or_default()
            .replace("CMD", "")
    );
}
