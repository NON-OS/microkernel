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

//! Asid-scoped TLB shootdown for every page-table mutation site in
//! the paging manager. Always issues the local `invlpg` first; on
//! multi-CPU runtime it then IPIs the peer CPUs running the same
//! asid (or every online CPU for a kernel-half flush). On single-CPU
//! runtime the broadcast block is skipped. An ack later than
//! `SHOOTDOWN_WARN_MS` is reported, the round is re-sent as an NMI to the
//! cpus still owing it (a target spinning with interrupts masked never takes
//! the vector), and it is waited for; timeout policy past
//! `SHOOTDOWN_TIMEOUT_MS` is fail-hard: a stale TLB entry would back
//! freed DMA or MMIO, so an ack that never arrives stops every other cpu
//! with the panic NMI and halts the originator.
//!
//! Only a change to a present entry reaches here. Installing over an absent
//! entry invalidates locally and sends nothing (see
//! `PendingFlush::after_install`), and every mutation site hands its flush
//! back to be committed after `PAGING_MANAGER` is released, so no cpu waits
//! for acknowledgements while holding the manager lock.

mod broadcast;
mod flush;
mod handle;
mod nudge;
mod report;
mod request;
mod select;
mod send;
mod wait;

pub use flush::{flush_tlb_all_smp, flush_tlb_one_smp, flush_tlb_range_smp};
pub use handle::{handle_shootdown_ipi, shootdown_in_flight};
pub use request::ASID_KERNEL;
