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

use super::spin_verdict::{judge, Holder};
use super::spin_watch::CHAIN_SPAN;

// A process kernel stack ending at TOP, and a frame well inside it.
const TOP: usize = 0xFFFF_8000_0010_8000;
const WAITER: usize = TOP - 0x2000;

#[test]
fn a_holder_anywhere_on_the_waiters_own_stack_is_a_re_take() {
    // orphans::reclaim held GATE and reclaim_dma, inlined into it, waited:
    // one merged frame, the two marks in either order.
    assert_eq!(judge(WAITER + 0x18, WAITER, TOP, false), Holder::ReTaken);
    assert_eq!(judge(WAITER - 0x18, WAITER, TOP, false), Holder::ReTaken);
    assert_eq!(judge(WAITER, WAITER, TOP, false), Holder::ReTaken);
    assert_eq!(judge(TOP - 1, WAITER, TOP, false), Holder::ReTaken);
    assert_eq!(judge(TOP - CHAIN_SPAN, WAITER, TOP, false), Holder::ReTaken);
    // The waiter at the stack's lowest byte is still on it.
    assert_eq!(judge(TOP - 8, TOP - CHAIN_SPAN, TOP, false), Holder::ReTaken);
}

#[test]
fn a_hold_on_any_other_stack_is_never_a_re_take() {
    // Another CPU's stack, just above or just below this one, or far away.
    assert_eq!(judge(TOP, WAITER, TOP, false), Holder::Other);
    assert_eq!(judge(TOP + 0x100, WAITER, TOP, false), Holder::Other);
    assert_eq!(judge(TOP - CHAIN_SPAN - 1, WAITER, TOP, false), Holder::Other);
    assert_eq!(judge(0xFFFF_8000_0090_0000, WAITER, TOP, false), Holder::Other);
}

#[test]
fn a_readers_take_is_at_most_likely() {
    // The last reader may have let go while another reader holds on.
    assert_eq!(judge(WAITER + 0x18, WAITER, TOP, true), Holder::Likely);
    assert_eq!(judge(0xFFFF_8000_0090_0000, WAITER, TOP, true), Holder::Other);
}

#[test]
fn off_a_known_stack_the_waiter_only_says_likely() {
    // Unknown bounds, the top itself (one past the stack), or a boot stack.
    assert_eq!(judge(WAITER + 0x18, WAITER, 0, false), Holder::Likely);
    assert_eq!(judge(TOP + 0x80, TOP, TOP, false), Holder::Likely);
    let away = TOP + 0x10_0000;
    assert_eq!(judge(away - 0x80, away, TOP, false), Holder::Likely);
    assert_eq!(judge(away + CHAIN_SPAN - 1, away, TOP, false), Holder::Likely);
    assert_eq!(judge(away + CHAIN_SPAN, away, TOP, false), Holder::Other);
    assert_eq!(judge(away - CHAIN_SPAN, away, TOP, false), Holder::Other);
    let below = TOP - CHAIN_SPAN - 0x100;
    assert_eq!(judge(below + 0x80, below, TOP, false), Holder::Likely);
}
