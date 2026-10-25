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
mod ack_irq;
pub(crate) mod bringup;
pub(crate) mod command;
mod dcbaa_slot;
pub(crate) mod drain_events;
mod interrupt_out;
mod isoch;
pub(crate) mod park;
mod port;
pub(crate) mod ring_doorbell;
mod transfer;
pub use ack_irq::ack_irq;
pub use bringup::*;
pub use command::*;
pub use dcbaa_slot::{clear_dcbaa_slot, set_dcbaa_slot};
pub use drain_events::drain_events;
pub use interrupt_out::interrupt_out;
pub use isoch::{isoch_endpoint, isoch_in, isoch_out, write_isoch_input, IsochEndpoint, IsochPipe};
pub use port::{power_all_ports, reset_port};
// Reached by the host proofs (userland/xhci_proofs), not by the capsule.
#[allow(unused_imports)]
pub use port::{port_action, PortAction};
// Reached by the host proofs (userland/xhci_proofs), not by the capsule.
#[allow(unused_imports)]
pub use isoch::{batch_fits, burst_fields, in_batch_fits, lengths, pack, td_bytes};
pub use transfer::*;
#[allow(unused_imports)]
pub use transfer::{complete, moved, short_on, stopped_early};
