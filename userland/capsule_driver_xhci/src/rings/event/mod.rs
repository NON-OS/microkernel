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
mod advance;
mod current_dequeue_phys;
mod current_trb;
mod drained_total;
mod erst_base_phys;
mod finish;
mod has_event;
mod issued_transfer;
mod orphan;
mod park;
mod parked;
mod parked_forget;
mod parked_ops;
mod state;
pub use issued_transfer::IssuedTransfer;
// The table's sizes are reached by the host proofs, not by the capsule.
#[allow(unused_imports)]
pub use parked::{ARMED, PARKED};
pub use state::EventRing;
