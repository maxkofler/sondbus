use crate::slave::BusState;

#[derive(PartialEq, Debug)]
pub enum CommandResponse {}

#[derive(PartialEq, Debug)]
pub enum ResponseState {
    /// Transmit the command byte
    Command(CommandResponse),
    /// Transmit the payload data
    Payload(CommandResponse),
    /// Transmit the CRC
    CRC(u8),
}

impl ResponseState {
    /// Poll the response for the next byte
    /// # Returns
    /// A tuple of the next bus state and a possible response byte
    pub fn tx(self) -> (BusState, Option<u8>) {
        (BusState::Idle, None)
    }
}

#[cfg(test)]
mod tests {}
