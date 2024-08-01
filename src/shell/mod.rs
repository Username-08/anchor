use core::panic;
pub mod element;

use smithay::{
    backend::renderer::utils::on_commit_buffer_handler,
    output::Output,
    reexports::wayland_server::protocol::{
        wl_buffer::WlBuffer, wl_output::WlOutput,
        wl_surface::WlSurface,
    },
    wayland::{
        buffer::BufferHandler,
        compositor::{
            get_parent, is_sync_subsurface, CompositorHandler,
        },
        shell::wlr_layer::{
            Layer, LayerSurface, WlrLayerShellHandler,
        },
    },
};

use crate::{
    data::ClientState,
    state::{AnchorState, BackendData},
};

use self::element::WindowElement;

impl<Backend: BackendData + 'static> CompositorHandler
    for AnchorState<Backend>
{
    fn compositor_state(
        &mut self,
    ) -> &mut smithay::wayland::compositor::CompositorState {
        &mut self.compositor_state
    }

    fn client_compositor_state<'a>(
        &self,
        client: &'a smithay::reexports::wayland_server::Client,
    ) -> &'a smithay::wayland::compositor::CompositorClientState {
        if let Some(state) = client.get_data::<ClientState>() {
            return &state.compositor_state;
        };
        panic!("unknown client type");
    }

    fn commit(
        &mut self,
        surface: &smithay::reexports::wayland_server::protocol::wl_surface::WlSurface,
    ) {
        on_commit_buffer_handler::<Self>(surface);
        self.backend_data.early_import(surface);

        if !is_sync_subsurface(surface) {
            let mut root = surface.clone();
            while let Some(parent) = get_parent(&root) {
                root = parent;
            }
            if let Some(window) = self.window_for_surface(&root) {
                window.0.on_commit();
            }
        }
        self.popups.commit(surface);

        // ens
    }
}

impl<Backend: BackendData> WlrLayerShellHandler
    for AnchorState<Backend>
{
    fn shell_state(
        &mut self,
    ) -> &mut smithay::wayland::shell::wlr_layer::WlrLayerShellState
    {
        &mut self.layer_shell_state
    }

    fn new_layer_surface(
        &mut self,
        surface: LayerSurface,
        output: Option<WlOutput>,
        layer: Layer,
        namespace: String,
    ) {
        todo!(); // TODO
                 // let output = output
                 //     .as_ref()
                 //     .and_then(Output::from_resource)
                 //     .unwrap_or_else(|| {
                 //         self.space.outputs().next().unwrap().clone()
                 //     });
                 // let mut map = layer_map_for_output(&output);
                 // map.map_layer(&LayerSurface::new(surface, namespace))
                 //     .unwrap();
    }
}

impl<Backend: BackendData + 'static> AnchorState<Backend> {
    pub fn window_for_surface(
        &self,
        surface: &WlSurface,
    ) -> Option<WindowElement> {
        todo!(); // TODO
                 // self.space
                 //     .elements()
                 //     .find(|window| {
                 //         window
                 //             .wl_surface()
                 //             .map(|s| s == *surface)
                 //             .unwrap_or(false)
                 //     })
                 //     .cloned()
    }
}
