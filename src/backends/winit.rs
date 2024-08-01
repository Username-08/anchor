use crate::state::{self, AnchorState};
use smithay::{
    backend::{
        renderer::{damage::OutputDamageTracker, gles::GlesRenderer},
        winit::{self, WinitGraphicsBackend},
    },
    output::{Mode, Output, PhysicalProperties, Subpixel},
    reexports::{calloop::EventLoop, wayland_server::Display},
    wayland::dmabuf::{DmabufFeedback, DmabufGlobal, DmabufState},
};

pub struct WinitData {
    pub backend: WinitGraphicsBackend<GlesRenderer>,
    pub damage_tracker: OutputDamageTracker,
    pub dmabuf_state:
        (DmabufState, DmabufGlobal, Option<DmabufFeedback>),
    pub full_redraw: u8,
}

impl WinitData {
    pub fn new(
        backend: WinitGraphicsBackend<GlesRenderer>,
        damage_tracker: OutputDamageTracker,
        dmabuf_state: (
            DmabufState,
            DmabufGlobal,
            Option<DmabufFeedback>,
        ),
        full_redraw: u8,
    ) -> Self {
        Self {
            backend,
            damage_tracker,
            dmabuf_state,
            full_redraw,
        }
    }
}

impl state::BackendData for WinitData {
    fn seat_name(&self) -> String {
        String::from("anchor")
    }

    fn reset_buffers(&mut self, output: &smithay::output::Output) {
        self.full_redraw = 4;
    }

    fn early_import(
        &mut self,
        surface: &smithay::reexports::wayland_server::protocol::wl_surface::WlSurface,
    ) {
    }
}

pub fn run_winit() {
    let mut event_loop = EventLoop::try_new().unwrap();
    let display = Display::new().unwrap();
    let mut display_handle = display.handle();

    let (mut backend, mut winit) = match winit::init::<GlesRenderer>()
    {
        Ok(ret) => ret,
        Err(err) => {
            eprintln!("Failed to initialize Winit backend: {}", err);
            return;
        }
    };

    let size = backend.window_size();

    let mode = Mode {
        size,
        refresh: 60_000,
    };
    let output = Output::new(
        "anchor".into(),
        PhysicalProperties {
            size: (0, 0).into(),
            subpixel: Subpixel::Unknown,
            make: "Smithay".into(),
            model: "anchor".into(),
        },
    );
    let _global = output
        .create_global::<AnchorState<WinitData>>(&display_handle);
    output.change_current_state(
        Some(mode),
        Some(smithay::utils::Transform::Flipped180),
        None,
        Some((0, 0).into()),
    );
    output.set_preferred(mode);
}
