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

use super::rule::*;

// A process kernel stack ending at TOP, and a frame well inside it.
const TOP: usize = 0xFFFF_8000_0010_8000;
const WAITER: usize = TOP - 0x2000;

#[test]
fn the_holder_above_on_its_own_stack_is_a_re_take() {
    // orphans::reclaim held GATE, reclaim_dma deeper on the same stack waited.
    assert_eq!(judge(WAITER + 0x180, WAITER, TOP), Holder::ReTaken);
    assert_eq!(judge(TOP - 1, WAITER, TOP), Holder::ReTaken);
    assert_eq!(judge(TOP - 8, TOP - CHAIN_SPAN, TOP), Holder::ReTaken);
}

#[test]
fn a_hold_on_any_other_stack_is_never_a_re_take() {
    // Another CPU's stack, just above this one or anywhere else.
    assert_eq!(judge(TOP, WAITER, TOP), Holder::Other);
    assert_eq!(judge(TOP + 0x100, WAITER, TOP), Holder::Other);
    assert_eq!(judge(TOP - CHAIN_SPAN - 8, WAITER, TOP), Holder::Other);
    assert_eq!(judge(0xFFFF_8000_0090_0000, WAITER, TOP), Holder::Other);
    // A frame below the waiter's has already returned.
    assert_eq!(judge(WAITER - 0x40, WAITER, TOP), Holder::Other);
    assert_eq!(judge(WAITER, WAITER, TOP), Holder::Other);
}

#[test]
fn off_its_known_stack_the_waiter_only_says_likely() {
    // Unknown bounds, or a waiter on a boot or interrupt stack.
    assert_eq!(judge(WAITER + 0x180, WAITER, 0), Holder::Likely);
    // The top itself is one past the stack, not on it.
    assert_eq!(judge(TOP + 0x80, TOP, TOP), Holder::Likely);
    let away = TOP + 0x10_0000;
    assert_eq!(judge(away + 0x180, away, TOP), Holder::Likely);
    assert_eq!(judge(away + CHAIN_SPAN - 1, away, TOP), Holder::Likely);
    assert_eq!(judge(away + CHAIN_SPAN, away, TOP), Holder::Other);
    assert_eq!(judge(away, away, TOP), Holder::Other);
    let below = TOP - CHAIN_SPAN - 0x100;
    assert_eq!(judge(below + 0x80, below, TOP), Holder::Likely);
}
