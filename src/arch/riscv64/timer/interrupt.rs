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

use super::deadline::{default_tick_delta, set_next_timer};

/// `from_user`: sstatus.SPP was clear, so the trap came from U-mode
/// (privileged spec, sstatus).
pub fn handle_timer_interrupt(from_user: bool) {
    // Before the tick runs: its dead-thread switch and band wake act only on
    // a tick that interrupted U-mode, which holds no kernel lock.
    crate::process::accounting::set_tick_origin(from_user);
    if set_next_timer(default_tick_delta()).is_err() {
        super::super::cpu::halt();
    }
    if super::super::cpu::csr::clear_csr(
        super::super::cpu::csr::SIP,
        super::super::cpu::csr::SIP_STIP,
    )
    .is_err()
    {
        super::super::cpu::halt();
    }
    crate::process::scheduler::preemption::tick();
    // Queued teardowns finish here, as on the other two arches, under the
    // same rule: only a tick from U-mode or the idle halt holds no lock.
    crate::process::exit::reclaim_at_tick(from_user);
}
