use crate::master::CommandDescriptor;
use std::fmt::Debug;

#[cfg(feature = "master-transport-serial")]
pub mod serial;

#[derive(Debug)]
pub enum MasterTransportError {}

pub trait MasterTransport {
    fn cycle_single(
        &mut self,
        command: CommandDescriptor,
    ) -> Result<CommandDescriptor, MasterTransportError>;
}
