use frame::{Handler, SlaveState};
use replace_with::replace_with_or_abort_unchecked;

mod frame;

pub struct SlaveCore {}

pub struct Slave {
    state: SlaveState,
    core: SlaveCore,
}

impl Slave {
    pub fn rx(&mut self, byte: u8) -> Option<u8> {
        let mut ret = None;

        unsafe {
            replace_with_or_abort_unchecked(&mut self.state, |state| {
                let response = state.rx(byte, &mut self.core);
                ret = response.response;
                response.state
            })
        };

        ret
    }
}
