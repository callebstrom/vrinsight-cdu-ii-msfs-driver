use serialport::{self, DataBits, FlowControl, Parity, SerialPortSettings, StopBits};
use std::time::Duration;

const RESET_COMMAND: &[u8] = &[0x43, 0x4d, 0x44, 0x52, 0x53, 0x54, 0x00, 0x00]; // CMDRST..
const CONNECT_COMMAND: &[u8] = &[0x43, 0x4d, 0x44, 0x43, 0x4f, 0x4e, 0x00, 0x00]; // CMDCON..
const GET_FUNCTION_COMMAND: &[u8] = "CMDFUN.O".as_bytes();
const GET_VERSION_COMMAND: &[u8] = "CMDVER.O".as_bytes();

pub struct CDU {
    port: Box<dyn serialport::SerialPort>,
}

impl CDU {
    pub fn new(port_name: &str) -> Self {
        let mut port = Self::connect_serial(port_name);
        Self::configure(&mut *port);

        return Self { port };
    }

    pub fn keep_alive(&mut self) {
        self.port
            .write(GET_VERSION_COMMAND)
            .expect("Failed to send keep-alive");

        loop {
            if Self::get_command_response(self.port.as_mut()).is_ok() {
                break;
            }
        }
    }

    fn wait_for_port(port_name: &str) -> serialport::SerialPortInfo {
        loop {
            let ports = serialport::available_ports().expect("Could not discover ports");

            let maybe_port = ports.into_iter().find(|p| p.port_name == port_name);
            if maybe_port.is_some() {
                return maybe_port.expect("Could not find port");
            }

            std::thread::sleep(Duration::from_secs(1));
        }

    }

    fn connect_serial(port_name: &str) -> Box<dyn serialport::SerialPort> {
        
        log::info!("Waiting for COM port");
        let port = Self::wait_for_port(port_name);

        let port = serialport::open_with_settings(
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

        log::info!("Connected to serial device on COM7");
        return port;
    }

    fn get_command_response(
        port: &mut dyn serialport::SerialPort,
    ) -> Result<Vec<u8>, std::io::Error> {
        let mut buf: Vec<u8> = vec![0; 8];
        return port.read_exact(buf.as_mut()).map(|_| buf);
    }

    fn configure(port: &mut dyn serialport::SerialPort) -> () {
        log::trace!("Configuring CDU II");

        port.write(RESET_COMMAND).expect("Could not reset device");
        std::thread::sleep(Duration::from_secs(5));

        port.write(CONNECT_COMMAND)
            .expect("Could not connect to device");
        std::thread::sleep(Duration::from_secs(5));

        Self::get_command_response(port).expect("Could not reset device");

        log::info!("Successfully reset CDU II");

        port.write(GET_FUNCTION_COMMAND)
            .expect("Could not get device type");

        let device_type = Self::get_command_response(port).expect("Could not reset device");

        log::info!(
            "Detected device type: {}",
            String::from_utf8(device_type)
                .unwrap_or_default()
                .replace("CMD", "")
        );
    }

    pub fn read<'life>(&mut self) -> Result<String, std::io::Error> {
        let mut buf: Vec<u8> = vec![12; 8];

        return self
            .port
            .read_exact(buf.as_mut())
            .map(|()| String::from_utf8(buf).expect("Could read parse CDU data"))
            .map(|key| key.split("\0").next().expect("").to_string());
    }
}
