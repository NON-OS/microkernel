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

//! The controller files that talk to registers and host DMA memory and
//! nothing else.
//!
//! Each is the shipping file. `park` is public here because `reset_port`
//! reaches it as `crate::controller::park`, the same path it has in the
//! capsule. The handoff's two files are included as siblings so `claim`
//! resolves through `super` exactly as it does in the capsule. The two
//! completion waits are public modules so a test can call them by the path
//! their siblings use; in the capsule they are reached the same way.

#[path = "../../../capsule_driver_xhci/src/controller/bulk/mod.rs"]
mod bulk;
#[path = "../../../capsule_driver_xhci/src/controller/legacy_handoff/claim.rs"]
mod claim;
#[path = "../../../capsule_driver_xhci/src/controller/drain_events.rs"]
mod drain_events;
#[path = "../../../capsule_driver_xhci/src/controller/get_device_descriptor.rs"]
mod get_device_descriptor;
#[path = "../../../capsule_driver_xhci/src/controller/halt.rs"]
mod halt;
#[path = "../../../capsule_driver_xhci/src/controller/issue_address_device.rs"]
mod issue_address_device;
#[path = "../../../capsule_driver_xhci/src/controller/issue_enable_slot.rs"]
mod issue_enable_slot;
#[path = "../../../capsule_driver_xhci/src/controller/legacy_handoff/legacy_handoff.rs"]
mod legacy_handoff;
#[path = "../../../capsule_driver_xhci/src/controller/park.rs"]
pub mod park;
#[path = "../../../capsule_driver_xhci/src/controller/poll_interrupt_in/mod.rs"]
mod poll_interrupt_in;
#[path = "../../../capsule_driver_xhci/src/controller/program_command_ring.rs"]
mod program_command_ring;
#[path = "../../../capsule_driver_xhci/src/controller/program_event_ring.rs"]
mod program_event_ring;
#[path = "../../../capsule_driver_xhci/src/controller/refuse_unsupported.rs"]
mod refuse_unsupported;
#[path = "../../../capsule_driver_xhci/src/controller/recover_endpoint.rs"]
mod recover_endpoint;
#[path = "../../../capsule_driver_xhci/src/controller/reset.rs"]
mod reset;
#[path = "../../../capsule_driver_xhci/src/controller/reset_port.rs"]
mod reset_port;
#[path = "../../../capsule_driver_xhci/src/controller/reset_toggle.rs"]
mod reset_toggle;
#[path = "../../../capsule_driver_xhci/src/controller/ring_doorbell.rs"]
mod ring_doorbell;
#[path = "../../../capsule_driver_xhci/src/controller/run_command.rs"]
mod run_command;
#[path = "../../../capsule_driver_xhci/src/controller/scratchpad.rs"]
mod scratchpad;
#[path = "../../../capsule_driver_xhci/src/controller/start.rs"]
mod start;
#[path = "../../../capsule_driver_xhci/src/controller/wait_cnr_clear.rs"]
mod wait_cnr_clear;
#[path = "../../../capsule_driver_xhci/src/controller/wait_command_completion.rs"]
pub mod wait_command_completion;
#[path = "../../../capsule_driver_xhci/src/controller/wait_hc_running.rs"]
mod wait_hc_running;
#[path = "../../../capsule_driver_xhci/src/controller/wait_transfer_completion.rs"]
pub mod wait_transfer_completion;

pub use bulk::{bulk_transfer, reset_bulk_endpoint, write_bulk_input, BulkEndpoint, BulkPipes};
pub use drain_events::{drain_events, DRAIN_BATCH};
pub use get_device_descriptor::{
    get_device_descriptor, read_device_descriptor, DEVICE_DESCRIPTOR_LEN, DEVICE_DESCRIPTOR_PREFIX,
};
pub use halt::halt;
pub use issue_address_device::{issue_address_device, SET_ADDRESS_SETTLE_MS};
pub use issue_enable_slot::issue_enable_slot;
pub use legacy_handoff::legacy_handoff;
pub use poll_interrupt_in::{poll_interrupt_in, IntrPoll};
pub use program_command_ring::program_command_ring;
pub use program_event_ring::program_event_ring;
pub use refuse_unsupported::refuse_unsupported;
pub use recover_endpoint::{recover_endpoint, Recovered, CC_CONTEXT_STATE_ERROR};
pub use reset::reset;
pub use reset_port::{port_action, power_all_ports, reset_port, PortAction};
pub use run_command::run_command;
pub use scratchpad::{aligned_page, Scratchpads};
pub use start::start;
pub use wait_cnr_clear::wait_cnr_clear;
pub use wait_hc_running::wait_hc_running;
