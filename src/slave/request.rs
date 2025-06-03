use crate::slave::BusState;

#[derive(PartialEq, Debug)]
pub enum CommandRequest {
    NOP,
    SYN,
    BWQ,
    PRQ,
    PWQ,
    XRS,
    XWS,
}

#[derive(PartialEq, Debug)]
pub enum RequestState {
    /// Wait for the command byte to be received
    Command,
    /// Wait for and handle the payload of the command
    Payload(CommandRequest),
    /// Wait for the CRC to be received
    CRC(CommandRequest, u8),
}

impl RequestState {
    /// Handle an incoming byte from the bus
    /// # Arguments
    /// * `data` - The byte of data to process
    /// # Returns
    /// A tuple of the next bus state and a possible response byte
    pub fn rx(self, _data: u8) -> (BusState, Option<u8>) {
        (BusState::Idle, None)
    }
}

#[cfg(test)]
mod tests {}
