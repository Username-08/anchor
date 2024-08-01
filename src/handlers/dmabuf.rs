use smithay::{
    backend::renderer::ImportDma,
    delegate_dmabuf,
    reexports::wayland_server::protocol::wl_buffer::WlBuffer,
    wayland::{buffer::BufferHandler, dmabuf::DmabufHandler},
};

use crate::{
    backends::winit::WinitData,
    state::{self, AnchorState, BackendData},
};

impl<Backend: BackendData> BufferHandler
    for state::AnchorState<Backend>
{
    fn buffer_destroyed(&mut self, buffer: &WlBuffer) {}
}

impl DmabufHandler for state::AnchorState<WinitData> {
    fn dmabuf_state(
        &mut self,
    ) -> &mut smithay::wayland::dmabuf::DmabufState {
        &mut self.backend_data.dmabuf_state.0
    }

    fn dmabuf_imported(
        &mut self,
        global: &smithay::wayland::dmabuf::DmabufGlobal,
        dmabuf: smithay::backend::allocator::dmabuf::Dmabuf,
        notifier: smithay::wayland::dmabuf::ImportNotifier,
    ) {
        if self
            .backend_data
            .backend
            .renderer()
            .import_dmabuf(&dmabuf, None)
            .is_ok()
        {
            let _ = notifier
                .successful::<state::AnchorState<WinitData>>();
        } else {
            notifier.failed();
        }
    }
}

delegate_dmabuf!(AnchorState<WinitData>);
