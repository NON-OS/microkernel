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

//! libc's heap counters as the wrapper drives them: the size is what init
//! gave, a take raises the peak to at least what is held, and a give takes
//! it back. Values far past any test's own allocations, so the other tests
//! freeing through the wrapper at the same time cannot hide a change.

use super::stats::{gave, reset_peak, sized, stats, took};

const BIG: usize = 1 << 40;

#[test]
fn the_counters_follow_takes_and_gives() {
    sized(BIG);
    assert_eq!(stats().size, BIG);
    assert!(took(core::ptr::null_mut(), BIG).is_null() && stats().used < BIG);
    let before = stats().used;
    let block = core::ptr::NonNull::<u8>::dangling().as_ptr();
    assert_eq!(took(block, BIG), block);
    assert!(stats().used >= before + BIG / 2 && stats().peak >= BIG);
    gave(BIG);
    assert!(stats().used < BIG);
    reset_peak();
    assert!(stats().peak < BIG);
}
