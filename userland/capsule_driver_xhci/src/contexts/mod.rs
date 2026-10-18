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
mod configure_ep;
mod endpoint_reset;
mod endpoint_state;
mod ep0;
mod evaluate;
mod hub;
mod input;
mod interval;
pub(crate) mod payload;
mod route;
mod size;
pub(crate) mod slot;
mod slot_copy;
pub use configure_ep::{write_configure_endpoint_input, EndpointConfig};
pub use endpoint_reset::{write_endpoint_reset_input, Dequeue};
pub use endpoint_state::{endpoint_state, EP_STATE_HALTED};
pub use ep0::{
    ep0_max_packet, ep0_needs_descriptor, is_superspeed, max_packet_for_speed, SPEED_FULL,
    SPEED_HIGH, SPEED_LOW, SPEED_SUPER,
};
pub use evaluate::write_evaluate_ep0_input;
pub use hub::write_configure_hub_input;
pub use input::write_address_device_input;
pub use interval::interrupt_interval;
pub use payload::interrupt_payload;
pub use route::{parent_of, route_depth, Topology, MAX_HUB_PORTS, MAX_TIERS};
pub use size::{device_context_bytes, input_context_bytes};
pub use slot::{HubInfo, Place};
pub use slot_copy::copy_slot_context;
// Reached by the host proofs (userland/xhci_proofs), not by the capsule.
#[allow(unused_imports)]
pub use slot_copy::with_entries;
