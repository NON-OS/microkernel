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

use super::constants::{
    GUARD_PAGES, IST_STACKS_PER_CPU, IST_STACK_SIZE, KSTACK_SIZE, PAGE_SIZE, PERCPU_STRIDE,
};
use super::manager::stack_slots::stack_slot_offset;

fn slot(i: usize) -> (u64, u64) {
    let size = if i == 0 { KSTACK_SIZE } else { IST_STACK_SIZE } as u64;
    let base = stack_slot_offset(i);
    (base, base + size)
}

#[test]
fn no_guard_page_is_a_page_of_another_stack() {
    let guard = (GUARD_PAGES * PAGE_SIZE) as u64;
    let slots: Vec<_> = (0..=IST_STACKS_PER_CPU).map(slot).collect();
    for (i, &(base, top)) in slots.iter().enumerate() {
        let guards = [(base - guard, base), (top, top + guard)];
        for (j, &(b, t)) in slots.iter().enumerate() {
            if i == j {
                continue;
            }
            for &(gs, ge) in &guards {
                assert!(ge <= b || t <= gs, "guard of slot {i} overlaps slot {j}");
            }
        }
    }
    assert!(slots[0].0 >= guard);
    assert!(slots[IST_STACKS_PER_CPU].1 + guard <= PERCPU_STRIDE);
}
