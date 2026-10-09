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

//! The machine's tick count and the work paced by it.
//!
//! Every CPU takes the timer interrupt, each from its own LAPIC timer, but
//! the tick count is the machine's clock: counted on all of them it ran as
//! many times fast as there were CPUs online. So only the boot CPU calls
//! these. The rest of the tick handler is per CPU: its time slice, its
//! sleepers sweep and its preemption.

use super::hooks;
use super::state;

/*
 * The EWMA decay constants in the load-average module assume a five-second
 * sampling period; the LAPIC preemption timer runs at 100 Hz.
 */
const LOAD_SAMPLE_TICKS: u64 = 500;

/// Count one tick of the machine's clock.
pub(super) fn advance() {
    state::increment_ticks();
    if option_env!("NONOS_FBCONSOLE").is_some() {
        super::heartbeat::on_tick(state::get_ticks());
    }
}

/// The work that runs every so many ticks of the machine's clock.
pub(super) fn paced_work() {
    if state::get_ticks() % 10 == 0 {
        crate::process::alarm::tick();
        #[cfg(target_arch = "x86_64")]
        crate::arch::x86_64::acpi::power_button::poll(state::get_ticks());
    }

    if state::get_ticks() % LOAD_SAMPLE_TICKS == 0 {
        crate::fs::procfs::update_load_averages();
    }

    #[cfg(all(target_arch = "x86_64", feature = "nonos-arch-iommu"))]
    crate::arch::x86_64::iommu::unit::fault::poll_faults(state::get_ticks());

    hooks::invoke_hook();
}
