// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! What the tests call, re-exported at `crate::controller` as in the capsule.

pub use super::bulk::{
    bulk_transfer, reset_bulk_endpoint, write_bulk_input, BulkEndpoint, BulkPipes,
};
pub use super::drain_events::{drain_events, DRAIN_BATCH};
pub use super::get_device_descriptor::{
    get_device_descriptor, read_device_descriptor, DEVICE_DESCRIPTOR_LEN, DEVICE_DESCRIPTOR_PREFIX,
};
pub use super::halt::halt;
pub use super::isoch::{
    batch_fits, burst_fields, isoch_endpoint, isoch_out, write_isoch_input, IsochEndpoint,
    IsochPipe,
};
pub use super::issue_address_device::{issue_address_device, SET_ADDRESS_SETTLE_MS};
pub use super::issue_enable_slot::issue_enable_slot;
pub use super::legacy_handoff::legacy_handoff;
pub use super::poll_interrupt_in::{poll_interrupt_in, IntrPoll};
pub use super::program_command_ring::program_command_ring;
pub use super::program_event_ring::program_event_ring;
pub use super::recover_endpoint::{recover_endpoint, Recovered, CC_CONTEXT_STATE_ERROR};
pub use super::refuse_unsupported::refuse_unsupported;
pub use super::reset::reset;
pub use super::reset_port::{port_action, power_all_ports, reset_port, PortAction};
pub use super::run_command::run_command;
pub use super::scratchpad::{aligned_page, Scratchpads};
pub use super::start::start;
pub use super::wait_cnr_clear::wait_cnr_clear;
pub use super::wait_hc_running::wait_hc_running;
