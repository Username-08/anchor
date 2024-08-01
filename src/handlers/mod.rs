pub mod dmabuf;
pub mod seat;
use smithay::{
    delegate_data_control, delegate_data_device, delegate_output,
    delegate_primary_selection, delegate_shm,
    wayland::{
        output::OutputHandler,
        selection::{
            data_device::{
                ClientDndGrabHandler, DataDeviceHandler,
                ServerDndGrabHandler,
            },
            primary_selection::{
                PrimarySelectionHandler, PrimarySelectionState,
            },
            wlr_data_control::{
                DataControlHandler, DataControlState,
            },
            SelectionHandler,
        },
        shm::{ShmHandler, ShmState},
    },
};

use crate::state::{AnchorState, BackendData};

impl<Backend: BackendData> DataDeviceHandler
    for AnchorState<Backend>
{
    fn data_device_state(
        &self,
    ) -> &smithay::wayland::selection::data_device::DataDeviceState
    {
        &self.data_device_state
    }
}

impl<Backend: BackendData> ClientDndGrabHandler
    for AnchorState<Backend>
{
    fn started(
        &mut self,
        source: Option<smithay::reexports::wayland_server::protocol::wl_data_source::WlDataSource>,
        icon: Option<smithay::reexports::wayland_server::protocol::wl_surface::WlSurface>,
        seat: smithay::input::Seat<Self>,
    ) {
        self.dnd_icon = icon;
    }

    fn dropped(&mut self, seat: smithay::input::Seat<Self>) {
        self.dnd_icon = None;
    }
}

impl<Backend: BackendData> ServerDndGrabHandler
    for AnchorState<Backend>
{
    fn accept(
        &mut self,
        mime_type: Option<String>,
        seat: smithay::input::Seat<Self>,
    ) {
    }

    fn action(
        &mut self,
        action: smithay::reexports::wayland_server::protocol::wl_data_device_manager::DndAction,
        seat: smithay::input::Seat<Self>,
    ) {
    }

    fn dropped(&mut self, seat: smithay::input::Seat<Self>) {}

    fn cancelled(&mut self, seat: smithay::input::Seat<Self>) {}

    fn send(
        &mut self,
        mime_type: String,
        fd: std::os::unix::prelude::OwnedFd,
        seat: smithay::input::Seat<Self>,
    ) {
    }

    fn finished(&mut self, seat: smithay::input::Seat<Self>) {}
}

delegate_data_device!(@<Backend: BackendData> AnchorState<Backend>);

impl<Backend: BackendData> OutputHandler for AnchorState<Backend> {
    fn output_bound(
        &mut self,
        _output: smithay::output::Output,
        _wl_output: smithay::reexports::wayland_server::protocol::wl_output::WlOutput,
    ) {
    }
}

delegate_output!(@<Backend: BackendData> AnchorState<Backend>);

impl<Backend: BackendData> SelectionHandler for AnchorState<Backend> {
    type SelectionUserData = ();

    fn new_selection(
        &mut self,
        ty: smithay::wayland::selection::SelectionTarget,
        source: Option<smithay::wayland::selection::SelectionSource>,
        seat: smithay::input::Seat<Self>,
    ) {
        todo!("xwayland")
    }

    fn send_selection(
        &mut self,
        ty: smithay::wayland::selection::SelectionTarget,
        mime_type: String,
        fd: std::os::unix::prelude::OwnedFd,
        seat: smithay::input::Seat<Self>,
        user_data: &Self::SelectionUserData,
    ) {
        todo!("xwayland")
    }
}

impl<Backend: BackendData> PrimarySelectionHandler
    for AnchorState<Backend>
{
    fn primary_selection_state(&self) -> &PrimarySelectionState {
        &self.primary_selection_state
    }
}

delegate_primary_selection!(@<Backend: BackendData> AnchorState<Backend>);

impl<Backend: BackendData> DataControlHandler
    for AnchorState<Backend>
{
    fn data_control_state(&self) -> &DataControlState {
        &self.data_control_state
    }
}

delegate_data_control!(@<Backend: BackendData> AnchorState<Backend>);

impl<Backend: BackendData> ShmHandler for AnchorState<Backend> {
    fn shm_state(&self) -> &ShmState {
        &self.shm_state
    }
}

delegate_shm!(@<Backend: BackendData> AnchorState<Backend>);
