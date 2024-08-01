use smithay::{
    reexports::wayland_server::backend::ClientData,
    wayland::compositor::CompositorClientState,
};

pub struct ClientState {
    pub compositor_state: CompositorClientState,
}

impl ClientData for ClientState {
    fn initialized(
        &self,
        _client_id: smithay::reexports::wayland_server::backend::ClientId,
    ) {
    }

    fn disconnected(
        &self,
        _client_id: smithay::reexports::wayland_server::backend::ClientId,
        _reason: smithay::reexports::wayland_server::backend::DisconnectReason,
    ) {
    }

    fn debug(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        f.debug_struct("ClientData").finish_non_exhaustive()
    }
}
