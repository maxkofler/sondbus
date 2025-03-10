mod core;
mod state;

#[derive(Default, Debug)]
pub struct SlaveFrame {
    state: state::State,
    core: core::Core,
}

impl SlaveFrame {
    pub fn rx(mut self, data: u8) -> (Self, Option<u8>) {
        let response = self.state.rx(data, &mut self.core);

        self.state = response.state;

        (self, response.response)
    }
}

pub struct Response {
    state: state::State,
    response: Option<u8>,
}

pub trait Receiver {
    fn rx(self, data: u8, core: &mut core::Core) -> Response;
}
