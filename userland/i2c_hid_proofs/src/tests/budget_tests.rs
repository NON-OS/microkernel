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

//! A transfer call is waited for at least as long as the shipping
//! controller may take to answer it, on its longest path, by its own
//! constants (mounted from capsule_driver_i2c_pci by i2c_transfer_proofs).

use i2c_transfer_proofs::constants::{
    ENABLE_TIMEOUT_MS, IDLE_TIMEOUT_MS, TRANSFER_BASE_MS, TRANSFER_PER_KIB_MS,
};
use i2c_transfer_proofs::protocol::{TRANSFER_READ_MAX, TRANSFER_WRITE_MAX};

use crate::i2c_client::budget::call_timeout_ms;
use crate::i2c_client::transfer::call_timeout;

/// One disable (control/disable.rs): the abort, then the disable itself.
const DISABLE_MS: u64 = 2 * ENABLE_TIMEOUT_MS;

/// The longest the controller spends on one transfer of `bytes`
/// (transaction/engine/transfer.rs): the bus idle; set_target on a core a
/// failed enable left enabled, a disable and an enable; the enable; the run,
/// by its budget (engine/budget.rs); the disable; and the reset a second
/// fault in a row runs before the reply (recover.rs, init/bring_up.rs), a
/// disable again.
fn controller_worst_ms(bytes: usize) -> u64 {
    let set_target = DISABLE_MS + ENABLE_TIMEOUT_MS;
    let run = TRANSFER_BASE_MS + (bytes as u64 * TRANSFER_PER_KIB_MS).div_ceil(1024);
    IDLE_TIMEOUT_MS + set_target + ENABLE_TIMEOUT_MS + run + DISABLE_MS + DISABLE_MS
}

/// Each of the path's ten waits (transaction/pace.rs) can end one tick late
/// (TICK_HZ 100, src/arch/x86_64/interrupt/apic/preemption/install.rs), and
/// the controller can wait one slice of ten ticks to be scheduled
/// (src/process/scheduler/preemption/state.rs).
const SCHEDULING_MS: u64 = 10 * 10 + 10 * 10;

#[test]
fn every_transfer_is_waited_for_as_long_as_the_controller_may_take() {
    for bytes in 0..=TRANSFER_READ_MAX + TRANSFER_WRITE_MAX {
        let wait = call_timeout_ms(bytes);
        let worst = controller_worst_ms(bytes) + SCHEDULING_MS;
        assert!(wait >= worst, "{bytes} bytes: waited {wait} ms, the controller may take {worst}");
    }
}

#[test]
fn the_longest_path_without_its_scheduling_is_300_ms_and_the_bytes() {
    assert_eq!(controller_worst_ms(0), 300, "the path the budget is drawn from moved");
    assert_eq!(controller_worst_ms(2 + TRANSFER_READ_MAX), 300 + 251);
}

#[test]
fn a_transfer_waits_for_the_bytes_it_moves_both_ways() {
    let (write, read) = ([0u8; 4], [0u8; TRANSFER_READ_MAX]);
    let worst = controller_worst_ms(write.len() + read.len()) + SCHEDULING_MS;
    assert!(call_timeout(&write, &read) >= worst, "a 1 KiB read waited for its write alone");
}
