use smithay::{
    reexports::wayland_server::{backend, DisplayHandle},
    wayland::compositor::CompositorClientState,
};

use crate::state::State;

pub struct Data {
    pub display: DisplayHandle,
    pub state: State,
}

#[derive(Default)]
pub struct ClientData {
    pub compositor_state: CompositorClientState,
}

impl backend::ClientData for ClientData {}
