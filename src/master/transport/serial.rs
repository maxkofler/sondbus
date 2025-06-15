use std::path::Path;

use serial2::{CharSize, FlowControl, Parity, SerialPort, Settings, StopBits};

use crate::master::transport::{MasterTransport, MasterTransportError};

pub struct SerialTransport {
    port: SerialPort,
}

#[derive(Debug)]
pub enum SerialTransportError {}

impl SerialTransport {
    pub fn open(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let port = SerialPort::open(path, |mut settings: Settings| {
            settings.set_raw();
            settings.set_baud_rate(115200)?;
            settings.set_char_size(CharSize::Bits8);
            settings.set_stop_bits(StopBits::One);
            settings.set_parity(Parity::None);
            settings.set_flow_control(FlowControl::None);
            Ok(settings)
        })?;

        Ok(Self { port })
    }
}

impl MasterTransport for SerialTransport {
    fn cycle_single(
        &mut self,
        mut command: crate::master::CommandDescriptor,
    ) -> Result<crate::master::CommandDescriptor, MasterTransportError> {
        let instructions = command.to_instructions();

        self.port.write_all(&instructions.master_section).unwrap();

        let mut sections = Vec::new();
        for section in instructions.slave_sections {
            let mut data = vec![0u8; section.bytes];
            self.port.read_exact(&mut data[0..section.bytes]).unwrap();

            if section.crc {
                let mut data = [0u8];
                self.port.read_exact(&mut data).unwrap();
            }

            sections.push(data);
        }

        command.populate_from_slave_sections(sections);

        Ok(command)
    }
}
