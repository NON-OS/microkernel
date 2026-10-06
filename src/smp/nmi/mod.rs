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

//! What an NMI was sent for, and the sending of it.
//!
//! Two senders need a cpu that may be running with interrupts masked, where a
//! fixed vector would wait forever: a shootdown round a target has not
//! answered, and a fatal halt of the whole machine. Both leave their reason in
//! memory first and then raise the NMI; `on_nmi` reads it back.
//!
//! Everything reached from `on_nmi` is NMI-safe. The cpu is named by its APIC
//! id, never through GS, because an NMI can land in the kernel-entry window
//! before swapgs; nothing takes a lock, allocates or prints; and every step is
//! idempotent, so an NMI that interrupts the same work on its own cpu, or two
//! NMIs coalesced into one, do no harm. The gate runs on the per-cpu NMI stack
//! (`NMI_IST_INDEX`), and nothing on this path faults, so no nested IRET
//! re-opens NMI delivery while that stack is in use.

mod handle;
mod send;

pub use handle::on_nmi;
pub(crate) use send::{halt_others, kick};
