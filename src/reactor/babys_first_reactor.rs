use crate::{error::KiotoError, reactor::sys::EventQueue};

pub struct Reactor {
    tasks: EventQueue,
}

impl Reactor {
    pub fn new() -> Self {
        todo!()
    }
}
