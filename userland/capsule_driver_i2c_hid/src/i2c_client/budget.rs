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

//! How long a transfer call to driver.i2c_pci0 is waited for: as long as
//! the controller may spend on it on its longest path, and the scheduling
//! around that. A shorter wait gives up on a transfer the controller is
//! still finishing.
//!
//! The path, in capsule_driver_i2c_pci (transaction/engine/transfer.rs,
//! constants/bits.rs): the bus idle within 50 ms; set_target on a core a
//! failed enable left enabled, a disable (an abort of 25, then 25) and an
//! enable of 25; the enable, 25; the run, 50 and 250 a KiB moved
//! (engine/budget.rs); the disable, 25 and 25; and the reset a second fault
//! in a row runs before the reply (recover.rs, init/bring_up.rs), a disable
//! again, 25 and 25.

pub const CONTROLLER_FIXED_MS: u64 = 50 + (25 + 25 + 25) + 25 + 50 + (25 + 25) + (25 + 25);
pub const CONTROLLER_PER_KIB_MS: u64 = 250;
/// The path's ten waits (transaction/pace.rs) can each end a tick late, at
/// 100 Hz (src/arch/x86_64/interrupt/apic/preemption/install.rs), and the
/// controller can wait a slice of ten ticks to run
/// (src/process/scheduler/preemption/state.rs).
pub const SCHEDULING_MS: u64 = 10 * 10 + 10 * 10;

/// The wait for the answer to a transfer that moves `bytes`, both ways.
pub fn call_timeout_ms(bytes: usize) -> u64 {
    let moved = (bytes as u64).saturating_mul(CONTROLLER_PER_KIB_MS).div_ceil(1024);
    CONTROLLER_FIXED_MS + moved + SCHEDULING_MS
}
