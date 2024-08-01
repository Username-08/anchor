use std::sync::{Arc, Mutex};

use smithay::{
    delegate_compositor,
    desktop::{PopupManager, Space},
    input::{
        keyboard::Keysym,
        pointer::{CursorImageStatus, PointerHandle},
        Seat, SeatHandler, SeatState,
    },
    output::Output,
    reexports::wayland_server::{
        protocol::wl_surface::WlSurface, DisplayHandle,
    },
    utils::{Clock, Monotonic},
    wayland::{
        compositor::CompositorState,
        fractional_scale::FractionalScaleManagerState,
        keyboard_shortcuts_inhibit::KeyboardShortcutsInhibitState,
        output::OutputManagerState,
        presentation::PresentationState,
        selection::{
            data_device::DataDeviceState,
            primary_selection::PrimarySelectionState,
            wlr_data_control::DataControlState,
        },
        shell::{
            wlr_layer::WlrLayerShellState,
            xdg::{decoration::XdgDecorationState, XdgShellState},
        },
        shm::ShmState,
        viewporter::ViewporterState,
        xdg_activation::XdgActivationState,
        xdg_foreign::XdgForeignState,
    },
};

#[derive(Debug)]
pub struct AnchorState<Backend: BackendData + 'static> {
    pub backend_data: Backend,
    pub socket_name: Option<String>,
    pub display_handle: DisplayHandle,

    // smithay state
    pub compositor_state: CompositorState,
    pub data_device_state: DataDeviceState,
    pub layer_shell_state: WlrLayerShellState,
    pub output_manager_state: OutputManagerState,
    pub primary_selection_state: PrimarySelectionState,
    pub data_control_state: DataControlState,
    pub seat_state: SeatState<AnchorState<Backend>>,
    pub keyboard_shortcuts_inhibit_state:
        KeyboardShortcutsInhibitState,
    pub shm_state: ShmState,
    pub viewporter_state: ViewporterState,
    pub xdg_activation_state: XdgActivationState,
    pub xdg_decoration_state: XdgDecorationState,
    pub xdg_shell_state: XdgShellState,
    pub presentation_state: PresentationState,
    pub fractional_scale_manager_state: FractionalScaleManagerState,
    pub xdg_foreign_state: XdgForeignState,

    // desktop
    pub space: Space<crate::shell::element::WindowElement>,
    pub popups: PopupManager,

    pub dnd_icon: Option<WlSurface>,

    // input
    pub suppressed_keys: Vec<Keysym>,
    pub cursor_status: Arc<Mutex<CursorImageStatus>>,
    pub seat_name: String,
    pub seat: Seat<AnchorState<Backend>>,
    pub clock: Clock<Monotonic>,
    pub pointer: PointerHandle<AnchorState<Backend>>,
}

delegate_compositor!(@<Backend: BackendData + 'static> AnchorState<Backend>);

pub trait BackendData {
    const HAS_RELATIVE_MOTION: bool = false;
    const HAS_GESTURES: bool = false;
    fn seat_name(&self) -> String;
    fn reset_buffers(&mut self, output: &Output);
    fn early_import(&mut self, surface: &WlSurface);
    // fn update_led_state(&mut self, led_state: LedState);
}
