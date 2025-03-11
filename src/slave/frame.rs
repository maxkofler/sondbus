mod core;
mod state;

mod rx;
mod tx;

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

#[macro_export]
macro_rules! impl_receiver_nop {
    ($x: ty) => {
        impl $crate::slave::frame::Receiver for $x {
            fn rx(
                self,
                _data: u8,
                _core: &mut $crate::slave::frame::core::Core,
            ) -> $crate::slave::frame::Response {
                $crate::slave::frame::Response {
                    state: self.into(),
                    response: None,
                }
            }
        }
    };
}

pub trait Sender {
    fn tx(self, core: &mut core::Core) -> Response;
}

#[macro_export]
macro_rules! impl_sender_nop {
    ($x: ty) => {
        impl $crate::slave::frame::Sender for $x {
            fn tx(
                self,
                _core: &mut $crate::slave::frame::core::Core,
            ) -> $crate::slave::frame::Response {
                $crate::slave::frame::Response {
                    state: self.into(),
                    response: None,
                }
            }
        }
    };
}
