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

//! The Hyper-V interface's SynIC and hypercalls, for VMBus. The pure parts
//! (what the partition may use, the registers' values, the hypercall's
//! words, a message slot, and what the VMBus core may post) are proven in
//! kernel_proofs; `synic` and `hypercall` touch the machine.

mod features;
mod hypercall;
mod hypercall_abi;
mod message;
mod pages;
mod regs;
mod ring;
mod synic;
mod vmbus_out;

pub use hypercall_abi::{STATUS_INSUFFICIENT_BUFFERS, STATUS_SUCCESS};
pub use message::{MESSAGE_BYTES, PAYLOAD_MAX};
pub use regs::VECTOR_HV_SYNIC;
pub use synic::{events, init, post, ready, recv, service, signal, unmask, EVENT_WORDS};
pub use vmbus_out::{check, Refusal};
