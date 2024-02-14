use smithay::{
    backend::renderer::utils::on_commit_buffer_handler,
    delegate_compositor, delegate_data_device, delegate_output,
    delegate_seat, delegate_shm, delegate_xdg_shell,
    desktop::{Space, Window},
    input::{pointer::CursorImageStatus, SeatHandler, SeatState},
    reexports::wayland_server::protocol::wl_surface::WlSurface,
    utils::{Logical, Point},
    wayland::{
        buffer::BufferHandler,
        compositor::{
            get_parent, is_sync_subsurface, with_states,
            CompositorHandler, CompositorState,
        },
        output::OutputManagerState,
        selection::{
            data_device::{
                ClientDndGrabHandler, DataDeviceHandler,
                DataDeviceState, ServerDndGrabHandler,
            },
            SelectionHandler,
        },
        shell::xdg::{
            XdgShellHandler, XdgShellState, XdgToplevelSurfaceData,
        },
        shm::{ShmHandler, ShmState},
    },
};

use crate::data::ClientData;

pub struct State {
    pub compositor_state: CompositorState,
    pub data_device_state: DataDeviceState,
    pub seat_state: SeatState<Self>,
    pub shm_state: ShmState,
    pub space: Space<Window>,
    pub output_manager_state: OutputManagerState,
    pub xdg_shell_state: XdgShellState,
    pub cursor_status: CursorImageStatus,
    pub pointer_location: Point<f64, Logical>,
}

impl State {
    pub fn new(
        compositor_state: CompositorState,
        data_device_state: DataDeviceState,
        seat_state: SeatState<Self>,
        shm_state: ShmState,
        space: Space<Window>,
        output_manager_state: OutputManagerState,
        xdg_shell_state: XdgShellState,
        cursor_status: CursorImageStatus,
        pointer_location: Point<f64, Logical>,
    ) -> Self {
        Self {
            compositor_state,
            data_device_state,
            seat_state,
            shm_state,
            space,
            output_manager_state,
            xdg_shell_state,
            cursor_status,
            pointer_location,
        }
    }
}

impl SeatHandler for State {
    fn focus_changed(
        &mut self,
        _seat: &smithay::input::Seat<Self>,
        _focused: Option<&Self::KeyboardFocus>,
    ) {
    }

    fn cursor_image(
        &mut self,
        _seat: &smithay::input::Seat<Self>,
        _image: smithay::input::pointer::CursorImageStatus,
    ) {
    }

    type KeyboardFocus = WlSurface;

    type PointerFocus = WlSurface;

    fn seat_state(&mut self) -> &mut SeatState<Self> {
        &mut self.seat_state
    }
}

delegate_seat!(State);

impl CompositorHandler for State {
    fn compositor_state(&mut self) -> &mut CompositorState {
        &mut self.compositor_state
    }

    fn client_compositor_state<'a>(
        &self,
        client: &'a smithay::reexports::wayland_server::Client,
    ) -> &'a smithay::wayland::compositor::CompositorClientState {
        &client.get_data::<ClientData>().unwrap().compositor_state
    }

    fn commit(&mut self, surface: &WlSurface) {
        on_commit_buffer_handler::<Self>(surface);

        // check if surface is unsynchronized
        if !is_sync_subsurface(surface) {
            let mut root = surface.clone();
            while let Some(parent) = get_parent(&root) {
                root = parent;
            }
            if let Some(window) = self
                .space
                .elements()
                .find(|w| w.toplevel().wl_surface() == surface)
                .cloned()
            {
                window.on_commit();

                // send initial configuration
                let initial_configure_sent =
                    with_states(surface, |states| {
                        states
                            .data_map
                            .get::<XdgToplevelSurfaceData>()
                            .unwrap()
                            .lock()
                            .unwrap()
                            .initial_configure_sent
                    });

                if !initial_configure_sent {
                    window.toplevel().send_pending_configure();
                }
            }
        }
    }
}

delegate_compositor!(State);

impl BufferHandler for State {
    fn buffer_destroyed(
        &mut self,
        buffer: &smithay::reexports::wayland_server::protocol::wl_buffer::WlBuffer,
    ) {
    }
}

impl ShmHandler for State {
    fn shm_state(&self) -> &ShmState {
        &self.shm_state
    }
}

delegate_shm!(State);

impl ClientDndGrabHandler for State {}
impl ServerDndGrabHandler for State {}
impl SelectionHandler for State {
    type SelectionUserData = ();
}

impl DataDeviceHandler for State {
    fn data_device_state(&self) -> &DataDeviceState {
        &self.data_device_state
    }
}

delegate_data_device!(State);

impl XdgShellHandler for State {
    fn xdg_shell_state(&mut self) -> &mut XdgShellState {
        &mut self.xdg_shell_state
    }

    fn new_toplevel(
        &mut self,
        surface: smithay::wayland::shell::xdg::ToplevelSurface,
    ) {
        let window = Window::new(surface);
        self.space.map_element(window, (0, 0), false);
    }

    fn new_popup(
        &mut self,
        surface: smithay::wayland::shell::xdg::PopupSurface,
        positioner: smithay::wayland::shell::xdg::PositionerState,
    ) {
    }

    fn grab(
        &mut self,
        surface: smithay::wayland::shell::xdg::PopupSurface,
        seat: smithay::reexports::wayland_server::protocol::wl_seat::WlSeat,
        serial: smithay::utils::Serial,
    ) {
    }
}

delegate_xdg_shell!(State);

delegate_output!(State);
