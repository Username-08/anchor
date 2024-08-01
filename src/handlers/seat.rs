use std::borrow::Cow;

use smithay::{
    desktop::{LayerSurface, PopupKind, Window, WindowSurface},
    input::{
        keyboard::KeyboardTarget, pointer::PointerTarget,
        touch::TouchTarget, SeatHandler,
    },
    reexports::wayland_server::{
        backend::ObjectId, protocol::wl_surface::WlSurface,
    },
    utils::IsAlive,
    wayland::seat::WaylandFocus,
};

use crate::state::{self, AnchorState, BackendData};

impl<Backend: BackendData> SeatHandler for AnchorState<Backend> {
    type KeyboardFocus = KeyboardFocusTarget;

    type PointerFocus = PointerFocusTarget;

    fn seat_state(&mut self) -> &mut smithay::input::SeatState<Self> {
        todo!()
    }

    type TouchFocus = PointerFocusTarget;
}

#[derive(Debug, Clone, PartialEq)]
pub enum KeyboardFocusTarget {
    Window(Window),
    LayerSurface(LayerSurface),
    Popup(PopupKind),
}

impl IsAlive for KeyboardFocusTarget {
    fn alive(&self) -> bool {
        match self {
            KeyboardFocusTarget::Window(w) => w.alive(),
            KeyboardFocusTarget::LayerSurface(l) => l.alive(),
            KeyboardFocusTarget::Popup(p) => p.alive(),
        }
    }
}

impl<Backend: BackendData> KeyboardTarget<AnchorState<Backend>>
    for KeyboardFocusTarget
{
    fn enter(
        &self,
        seat: &smithay::input::Seat<AnchorState<Backend>>,
        data: &mut AnchorState<Backend>,
        keys: Vec<smithay::input::keyboard::KeysymHandle<'_>>,
        serial: smithay::utils::Serial,
    ) {
        match self {
            KeyboardFocusTarget::Window(w) => {
                match w.underlying_surface() {
                    WindowSurface::Wayland(w) => {
                        KeyboardTarget::enter(
                            w.wl_surface(),
                            seat,
                            data,
                            keys,
                            serial,
                        )
                    } // TODO: Add Xwayland support
                }
            }
            KeyboardFocusTarget::LayerSurface(l) => {
                KeyboardTarget::enter(
                    l.wl_surface(),
                    seat,
                    data,
                    keys,
                    serial,
                );
            }
            KeyboardFocusTarget::Popup(p) => {
                KeyboardTarget::enter(
                    p.wl_surface(),
                    seat,
                    data,
                    keys,
                    serial,
                );
            }
        }
    }

    fn leave(
        &self,
        seat: &smithay::input::Seat<AnchorState<Backend>>,
        data: &mut AnchorState<Backend>,
        serial: smithay::utils::Serial,
    ) {
        match self {
            KeyboardFocusTarget::Window(w) => {
                match w.underlying_surface() {
                    WindowSurface::Wayland(w) => {
                        KeyboardTarget::leave(
                            w.wl_surface(),
                            seat,
                            data,
                            serial,
                        )
                    } // TODO: Add Xwayland support
                }
            }
            KeyboardFocusTarget::LayerSurface(l) => {
                KeyboardTarget::leave(
                    l.wl_surface(),
                    seat,
                    data,
                    serial,
                );
            }
            KeyboardFocusTarget::Popup(p) => {
                KeyboardTarget::leave(
                    p.wl_surface(),
                    seat,
                    data,
                    serial,
                );
            }
        }
    }

    fn key(
        &self,
        seat: &smithay::input::Seat<AnchorState<Backend>>,
        data: &mut AnchorState<Backend>,
        key: smithay::input::keyboard::KeysymHandle<'_>,
        state: smithay::backend::input::KeyState,
        serial: smithay::utils::Serial,
        time: u32,
    ) {
        match self {
            KeyboardFocusTarget::Window(w) => {
                match w.underlying_surface() {
                    WindowSurface::Wayland(w) => KeyboardTarget::key(
                        w.wl_surface(),
                        seat,
                        data,
                        key,
                        state,
                        serial,
                        time,
                    ), // TODO: Add Xwayland support
                }
            }
            KeyboardFocusTarget::LayerSurface(l) => {
                KeyboardTarget::key(
                    l.wl_surface(),
                    seat,
                    data,
                    key,
                    state,
                    serial,
                    time,
                )
            }
            KeyboardFocusTarget::Popup(p) => KeyboardTarget::key(
                p.wl_surface(),
                seat,
                data,
                key,
                state,
                serial,
                time,
            ),
        }
    }

    fn modifiers(
        &self,
        seat: &smithay::input::Seat<AnchorState<Backend>>,
        data: &mut AnchorState<Backend>,
        modifiers: smithay::input::keyboard::ModifiersState,
        serial: smithay::utils::Serial,
    ) {
        match self {
            KeyboardFocusTarget::Window(w) => {
                match w.underlying_surface() {
                    WindowSurface::Wayland(w) => {
                        KeyboardTarget::modifiers(
                            w.wl_surface(),
                            seat,
                            data,
                            modifiers,
                            serial,
                        )
                    }
                }
            }
            KeyboardFocusTarget::LayerSurface(l) => {
                KeyboardTarget::modifiers(
                    l.wl_surface(),
                    seat,
                    data,
                    modifiers,
                    serial,
                )
            }
            KeyboardFocusTarget::Popup(p) => {
                KeyboardTarget::modifiers(
                    p.wl_surface(),
                    seat,
                    data,
                    modifiers,
                    serial,
                )
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum PointerFocusTarget {
    WlSurface(WlSurface),
    // SSD(SSD),
}

impl IsAlive for PointerFocusTarget {
    fn alive(&self) -> bool {
        match self {
            PointerFocusTarget::WlSurface(w) => w.alive(),
        }
    }
}

impl<Backend: BackendData> PointerTarget<AnchorState<Backend>>
    for PointerFocusTarget
{
    fn enter(
        &self,
        seat: &smithay::input::Seat<AnchorState<Backend>>,
        data: &mut AnchorState<Backend>,
        event: &smithay::input::pointer::MotionEvent,
    ) {
        match self {
            PointerFocusTarget::WlSurface(w) => {
                PointerTarget::enter(w, seat, data, event)
            }
        }
    }

    fn motion(
        &self,
        seat: &smithay::input::Seat<AnchorState<Backend>>,
        data: &mut AnchorState<Backend>,
        event: &smithay::input::pointer::MotionEvent,
    ) {
        match self {
            PointerFocusTarget::WlSurface(w) => {
                PointerTarget::motion(w, seat, data, event)
            }
        }
    }

    fn relative_motion(
        &self,
        seat: &smithay::input::Seat<AnchorState<Backend>>,
        data: &mut AnchorState<Backend>,
        event: &smithay::input::pointer::RelativeMotionEvent,
    ) {
        match self {
            PointerFocusTarget::WlSurface(w) => {
                PointerTarget::relative_motion(w, seat, data, event)
            }
        }
    }

    fn button(
        &self,
        seat: &smithay::input::Seat<AnchorState<Backend>>,
        data: &mut AnchorState<Backend>,
        event: &smithay::input::pointer::ButtonEvent,
    ) {
        match self {
            PointerFocusTarget::WlSurface(w) => {
                PointerTarget::button(w, seat, data, event)
            }
        }
    }

    fn axis(
        &self,
        seat: &smithay::input::Seat<AnchorState<Backend>>,
        data: &mut AnchorState<Backend>,
        frame: smithay::input::pointer::AxisFrame,
    ) {
        match self {
            PointerFocusTarget::WlSurface(w) => {
                PointerTarget::axis(w, seat, data, frame)
            }
        }
    }

    fn frame(
        &self,
        seat: &smithay::input::Seat<AnchorState<Backend>>,
        data: &mut AnchorState<Backend>,
    ) {
        match self {
            PointerFocusTarget::WlSurface(w) => {
                PointerTarget::frame(w, seat, data)
            }
        }
    }

    fn gesture_swipe_begin(
        &self,
        seat: &smithay::input::Seat<AnchorState<Backend>>,
        data: &mut AnchorState<Backend>,
        event: &smithay::input::pointer::GestureSwipeBeginEvent,
    ) {
        match self {
            PointerFocusTarget::WlSurface(w) => {
                PointerTarget::gesture_swipe_begin(
                    w, seat, data, event,
                )
            }
        }
    }

    fn gesture_swipe_update(
        &self,
        seat: &smithay::input::Seat<AnchorState<Backend>>,
        data: &mut AnchorState<Backend>,
        event: &smithay::input::pointer::GestureSwipeUpdateEvent,
    ) {
        match self {
            PointerFocusTarget::WlSurface(w) => {
                PointerTarget::gesture_swipe_update(
                    w, seat, data, event,
                )
            }
        }
    }

    fn gesture_swipe_end(
        &self,
        seat: &smithay::input::Seat<AnchorState<Backend>>,
        data: &mut AnchorState<Backend>,
        event: &smithay::input::pointer::GestureSwipeEndEvent,
    ) {
        match self {
            PointerFocusTarget::WlSurface(w) => {
                PointerTarget::gesture_swipe_end(w, seat, data, event)
            }
        }
    }

    fn gesture_pinch_begin(
        &self,
        seat: &smithay::input::Seat<AnchorState<Backend>>,
        data: &mut AnchorState<Backend>,
        event: &smithay::input::pointer::GesturePinchBeginEvent,
    ) {
        match self {
            PointerFocusTarget::WlSurface(w) => {
                PointerTarget::gesture_pinch_begin(
                    w, seat, data, event,
                )
            }
        }
    }

    fn gesture_pinch_update(
        &self,
        seat: &smithay::input::Seat<AnchorState<Backend>>,
        data: &mut AnchorState<Backend>,
        event: &smithay::input::pointer::GesturePinchUpdateEvent,
    ) {
        match self {
            PointerFocusTarget::WlSurface(w) => {
                PointerTarget::gesture_pinch_update(
                    w, seat, data, event,
                )
            }
        }
    }

    fn gesture_pinch_end(
        &self,
        seat: &smithay::input::Seat<AnchorState<Backend>>,
        data: &mut AnchorState<Backend>,
        event: &smithay::input::pointer::GesturePinchEndEvent,
    ) {
        match self {
            PointerFocusTarget::WlSurface(w) => {
                PointerTarget::gesture_pinch_end(w, seat, data, event)
            }
        }
    }

    fn gesture_hold_begin(
        &self,
        seat: &smithay::input::Seat<AnchorState<Backend>>,
        data: &mut AnchorState<Backend>,
        event: &smithay::input::pointer::GestureHoldBeginEvent,
    ) {
        match self {
            PointerFocusTarget::WlSurface(w) => {
                PointerTarget::gesture_hold_begin(
                    w, seat, data, event,
                )
            }
        }
    }

    fn gesture_hold_end(
        &self,
        seat: &smithay::input::Seat<AnchorState<Backend>>,
        data: &mut AnchorState<Backend>,
        event: &smithay::input::pointer::GestureHoldEndEvent,
    ) {
        match self {
            PointerFocusTarget::WlSurface(w) => {
                PointerTarget::gesture_hold_end(w, seat, data, event)
            }
        }
    }

    fn leave(
        &self,
        seat: &smithay::input::Seat<AnchorState<Backend>>,
        data: &mut AnchorState<Backend>,
        serial: smithay::utils::Serial,
        time: u32,
    ) {
        match self {
            PointerFocusTarget::WlSurface(w) => {
                PointerTarget::leave(w, seat, data, serial, time)
            }
        }
    }
}

impl<Backend: BackendData> TouchTarget<AnchorState<Backend>>
    for PointerFocusTarget
{
    fn down(
        &self,
        seat: &smithay::input::Seat<AnchorState<Backend>>,
        data: &mut AnchorState<Backend>,
        event: &smithay::input::touch::DownEvent,
        seq: smithay::utils::Serial,
    ) {
        match self {
            PointerFocusTarget::WlSurface(w) => {
                TouchTarget::down(w, seat, data, event, seq)
            }
        }
    }

    fn up(
        &self,
        seat: &smithay::input::Seat<AnchorState<Backend>>,
        data: &mut AnchorState<Backend>,
        event: &smithay::input::touch::UpEvent,
        seq: smithay::utils::Serial,
    ) {
        match self {
            PointerFocusTarget::WlSurface(w) => {
                TouchTarget::up(w, seat, data, event, seq)
            }
        }
    }

    fn motion(
        &self,
        seat: &smithay::input::Seat<AnchorState<Backend>>,
        data: &mut AnchorState<Backend>,
        event: &smithay::input::touch::MotionEvent,
        seq: smithay::utils::Serial,
    ) {
        match self {
            PointerFocusTarget::WlSurface(w) => {
                TouchTarget::motion(w, seat, data, event, seq)
            }
        }
    }

    fn frame(
        &self,
        seat: &smithay::input::Seat<AnchorState<Backend>>,
        data: &mut AnchorState<Backend>,
        seq: smithay::utils::Serial,
    ) {
        match self {
            PointerFocusTarget::WlSurface(w) => {
                TouchTarget::frame(w, seat, data, seq)
            }
        }
    }

    fn cancel(
        &self,
        seat: &smithay::input::Seat<AnchorState<Backend>>,
        data: &mut AnchorState<Backend>,
        seq: smithay::utils::Serial,
    ) {
        match self {
            PointerFocusTarget::WlSurface(w) => {
                TouchTarget::cancel(w, seat, data, seq)
            }
        }
    }

    fn shape(
        &self,
        seat: &smithay::input::Seat<AnchorState<Backend>>,
        data: &mut AnchorState<Backend>,
        event: &smithay::input::touch::ShapeEvent,
        seq: smithay::utils::Serial,
    ) {
        match self {
            PointerFocusTarget::WlSurface(w) => {
                TouchTarget::shape(w, seat, data, event, seq)
            }
        }
    }

    fn orientation(
        &self,
        seat: &smithay::input::Seat<AnchorState<Backend>>,
        data: &mut AnchorState<Backend>,
        event: &smithay::input::touch::OrientationEvent,
        seq: smithay::utils::Serial,
    ) {
        match self {
            PointerFocusTarget::WlSurface(w) => {
                TouchTarget::orientation(w, seat, data, event, seq)
            }
        }
    }
}

impl WaylandFocus for PointerFocusTarget {
    #[inline]
    fn wl_surface(&self) -> Option<Cow<'_, WlSurface>> {
        match self {
            PointerFocusTarget::WlSurface(w) => w.wl_surface(),
        }
    }
    #[inline]
    fn same_client_as(&self, object_id: &ObjectId) -> bool {
        match self {
            PointerFocusTarget::WlSurface(w) => {
                w.same_client_as(object_id)
            }
        }
    }
}

impl WaylandFocus for KeyboardFocusTarget {
    #[inline]
    fn wl_surface(&self) -> Option<Cow<'_, WlSurface>> {
        match self {
            KeyboardFocusTarget::Window(w) => w.wl_surface(),
            KeyboardFocusTarget::LayerSurface(l) => {
                Some(Cow::Borrowed(l.wl_surface()))
            }
            KeyboardFocusTarget::Popup(p) => {
                Some(Cow::Borrowed(p.wl_surface()))
            }
        }
    }
}
