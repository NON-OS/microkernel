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

use super::spin_watch::*;

#[test]
fn quiet_until_the_mark_then_once() {
    assert!(!due(5_000, 5_000 + REPORT_MS - 1, false));
    assert!(due(5_000, 5_000 + REPORT_MS, false));
    assert!(due(5_000, 5_000 + 60 * REPORT_MS, false));
    assert!(!due(5_000, 5_000 + REPORT_MS, true));
}

#[test]
fn a_clock_read_behind_the_start_is_no_wait() {
    // Two CPUs' clocks can disagree by a little; that is never a long wait.
    assert!(!due(5_000, 4_999, false));
    assert!(!due(u64::MAX, 0, false));
}

#[test]
fn the_window_is_one_process_kernel_stack() {
    assert_eq!(CHAIN_SPAN, super::user_constants::KERNEL_STACK_SIZE);
}

#[test]
fn neighbouring_locks_get_their_own_slots() {
    let base = 0xFFFF_8000_0200_0000usize;
    let mut seen = [false; SLOTS];
    for i in 0..SLOTS {
        let slot = slot_of(base + 8 * i);
        assert!(slot < SLOTS);
        assert!(!seen[slot], "two adjacent locks share slot {slot}");
        seen[slot] = true;
    }
    assert!((0..4096).all(|a| slot_of(usize::MAX - a) < SLOTS));
}
