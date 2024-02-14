mod cursor;
mod data;
mod state;
use std::{sync::Arc, time::Duration};

use smithay::{
    backend::{
        input::{InputEvent, KeyState, KeyboardKeyEvent},
        renderer::{
            damage::OutputDamageTracker,
            element::surface::WaylandSurfaceRenderElement,
            gles::GlesRenderer,
        },
        winit::{self, WinitEvent},
    },
    desktop::{space::render_output, Space},
    input::{
        keyboard::{keysyms, FilterResult},
        pointer::CursorImageStatus,
        SeatState,
    },
    output,
    reexports::{
        calloop::{
            generic::Generic,
            timer::{TimeoutAction, Timer},
            EventLoop, Interest, Mode, PostAction,
        },
        wayland_server::Display,
    },
    utils::{Transform, SERIAL_COUNTER},
    wayland::{
        compositor::CompositorState, output::OutputManagerState,
        selection::data_device::DataDeviceState,
        shell::xdg::XdgShellState, shm::ShmState,
        socket::ListeningSocketSource,
    },
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut display: Display<state::State> = Display::new()?;
    let dh = display.handle();

    let mut event_loop: EventLoop<data::Data> = EventLoop::try_new()?;

    let socket = ListeningSocketSource::new_auto()?;
    let socket_name = socket.socket_name().to_os_string();

    event_loop.handle().insert_source(
        socket,
        |stream, _, data| {
            data.display
                .insert_client(
                    stream,
                    Arc::new(data::ClientData::default()),
                )
                .unwrap();
        },
    )?;

    event_loop
        .handle()
        .insert_source(
            Generic::new(display, Interest::READ, Mode::Level),
            |_, display, data| {
                unsafe {
                    display
                        .get_mut()
                        .dispatch_clients(&mut data.state)
                        .unwrap();
                }
                Ok(PostAction::Continue)
            },
        )
        .unwrap();

    // compositor globals
    let compositor_state = CompositorState::new::<state::State>(&dh);
    let shm_state = ShmState::new::<state::State>(&dh, vec![]);
    let output_manager_state =
        OutputManagerState::new_with_xdg_output::<state::State>(&dh);

    let mut seat_state = SeatState::<state::State>::new();
    let mut seat = seat_state.new_wl_seat(&dh, "anchor_seat");
    let data_device_state = DataDeviceState::new::<state::State>(&dh);
    let space = Space::default();
    let xdg_shell_state = XdgShellState::new::<state::State>(&dh);
    seat.add_keyboard(Default::default(), 500, 500)?;
    seat.add_pointer();

    // compositor state
    let mut state = state::State::new(
        compositor_state,
        data_device_state,
        seat_state,
        shm_state,
        space,
        output_manager_state,
        xdg_shell_state,
        CursorImageStatus::default_named(),
        (0.0, 0.0).into(),
    );

    let mut data = data::Data { display: dh, state };

    // winit
    let (mut backend, mut winit) =
        winit::init::<GlesRenderer>().unwrap();

    let size = backend.window_size();
    let mode = output::Mode {
        size,
        refresh: 60_000,
    };

    let physical_properties = output::PhysicalProperties {
        size: (0, 0).into(),
        subpixel: output::Subpixel::Unknown,
        make: "anchor".into(),
        model: "anchor".into(),
    };

    let output =
        output::Output::new("anchor".into(), physical_properties);
    output.create_global::<state::State>(&data.display);
    output.change_current_state(
        Some(mode),
        Some(Transform::Flipped180),
        None,
        Some((0, 0).into()),
    );
    output.set_preferred(mode);

    data.state.space.map_output(&output, (0, 0));

    std::env::set_var("WAYLAND_DISPLAY", &socket_name);

    let start_time = std::time::Instant::now();
    let timer = Timer::immediate();

    let mut output_damage_tracker =
        OutputDamageTracker::from_output(&output);

    event_loop
        .handle()
        .insert_source(timer, move |_, _, data| {
            let display = &mut data.display;
            let state = &mut data.state;

            winit.dispatch_new_events(|event| {
                if let WinitEvent::Input(event) = event {
                    if let InputEvent::Keyboard { event } = event {
                        let serial = SERIAL_COUNTER.next_serial();
                        let time =
                            smithay::backend::input::Event::time_msec(
                                &event,
                            );
                        let press_state = event.state();
                        let action = seat
                            .get_keyboard()
                            .unwrap()
                            .input::<u8, _>(
                                state,
                                event.key_code(),
                                press_state,
                                serial,
                                time,
                                |_, _, handle| {
                                    let keysm: u32 =
                                        handle.modified_sym().into();
                                    if press_state
                                        == KeyState::Pressed
                                        && keysm
                                            == keysyms::KEY_t
                                                | keysyms::KEY_T
                                    {
                                        FilterResult::Intercept(1)
                                    } else {
                                        FilterResult::Forward
                                    }
                                },
                            );
                        if Some(1) == action {
                            std::process::Command::new("weston-terminal")
                                .spawn()
                                .unwrap();
                        }
                    }
                }
            });
            backend.bind().unwrap();
            let age = backend.buffer_age().unwrap_or(0);

            output_damage_tracker
                .render_output::<WaylandSurfaceRenderElement<GlesRenderer>, GlesRenderer>(
                    backend.renderer(),
                    age,
                    &[],
                    [0.1, 0.0, 0.0, 1.0],
                )
                .unwrap();

            render_output::<_, WaylandSurfaceRenderElement<GlesRenderer>, _, _>(
                &output,
                backend.renderer(),
                1.0,
                age,
                [&state.space],
                &[],
                &mut output_damage_tracker,
                [0.1, 0.0, 0.0, 1.0])
                .unwrap();

            backend.submit(None).unwrap();
            state.space.elements().for_each(|window| {
                window.send_frame(
                    &output,
                    start_time.elapsed(),
                    Some(Duration::ZERO),
                    |_, _| Some(output.clone()),
                )
            });
            state.space.refresh();
            display.flush_clients().unwrap();
            TimeoutAction::ToDuration(Duration::from_millis(16))
        })
        .unwrap();

    event_loop.run(None, &mut data, |_| {})?;

    Ok(())
}
