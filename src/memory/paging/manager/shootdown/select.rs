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

use super::eligible::{cpu_should_flush, may_hold_entries};

pub(super) const WORDS: usize = crate::smp::MAX_CPUS.div_ceil(64);

/// The cpus a round for `asid` must reach, how many, and how many of those
/// were still starting when read.
pub(super) fn select(asid: u32) -> ([u64; WORDS], u32, u32) {
    let self_cpu = crate::smp::cpu_id();
    let mut targets: u32 = 0;
    let mut starting: u32 = 0;
    let mut selected = [0u64; WORDS];
    /*
     * Every cpu slot, filtered by whether it is running. Not `0..cpus_online()`:
     * that is a population count, while cpu numbers are handed out once per AP
     * attempted and are not reused when one fails. With a single failed AP the
     * live numbers are sparse, so counting up to the population both targets a
     * slot that never started, which can never acknowledge, and skips a cpu
     * that is running, which never gets the IPI. The wait in `broadcast` always
     * reaches its deadline and halts the machine.
     */
    for cpu in 0..crate::smp::MAX_CPUS {
        if cpu == self_cpu || !may_hold_entries(cpu, asid) {
            continue;
        }
        let Some(d) = crate::smp::percpu::get(cpu) else {
            continue;
        };
        if !cpu_should_flush(d, asid) {
            continue;
        }
        selected[cpu / 64] |= 1u64 << (cpu % 64);
        targets += 1;
        starting += u32::from(!crate::smp::cpu_is_online(cpu));
    }
    (selected, targets, starting)
}
