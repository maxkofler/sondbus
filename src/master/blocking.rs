use crate::master::{
    transport::{MasterTransport, MasterTransportError},
    CommandDescriptor,
};

pub struct BlockingMaster<TRANSPORT: MasterTransport> {
    transport: TRANSPORT,
}

#[derive(Debug)]
pub enum BlockingMasterError {
    Transport(MasterTransportError),
}

impl<T: MasterTransport> BlockingMaster<T> {
    pub fn new(transport: T) -> Self {
        Self { transport }
    }

    pub fn syn(&mut self) -> Result<(), BlockingMasterError> {
        let command = CommandDescriptor::SYN;

        let _ = self.cycle_single(command)?;

        Ok(())
    }

    pub fn bwr(&mut self, offset: u8, data: Vec<u8>) -> Result<(), BlockingMasterError> {
        let command = CommandDescriptor::BWR { offset, data };

        self.cycle_single(command)?;

        Ok(())
    }

    pub fn lrd<const S: usize>(
        &mut self,
        address: u8,
        offset: u8,
    ) -> Result<[u8; S], BlockingMasterError> {
        let command = CommandDescriptor::LRD {
            address,
            offset,
            data: vec![0u8; S],
        };

        let res = self.cycle_single(command)?;

        if let CommandDescriptor::LRD {
            address,
            offset,
            data,
        } = res
        {
            let mut ret = [0u8; S];
            ret.copy_from_slice(&data[0..S]);
            Ok(ret)
        } else {
            panic!()
        }
    }

    fn cycle_single(
        &mut self,
        command: CommandDescriptor,
    ) -> Result<CommandDescriptor, BlockingMasterError> {
        let res = self.transport.cycle_single(command);
        match res {
            Ok(v) => Ok(v),
            Err(e) => Err(BlockingMasterError::Transport(e)),
        }
    }
}
