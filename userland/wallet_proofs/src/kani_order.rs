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

//! Kani harnesses for the escrow words the wallet reads from settle (B8,
//! v1.4.51 to v1.4.53): orderOf's state, deliveredAt and extra, a balance
//! word, the dispute window and the gas withdraw's size, for every input.
//! `order_tests/props.rs` checks the same claims on a million words.

use crate::landed::balance_of;
use crate::order::order_of::order_of;
use crate::order::refund_call::state_and_extra;

/// A word read as u64 only with its 24 high bytes zero; as u128 with 16.
fn tail_ok(w: &[u8; 32], from: usize) -> bool {
    let mut i = 0;
    while i < from {
        if w[i] != 0 {
            return false;
        }
        i += 1;
    }
    true
}

#[kani::proof]
#[kani::unwind(33)]
fn order_of_reads_only_small_words() {
    let w: [[u8; 32]; 5] = kani::any();
    if let Some((state, at)) = order_of(&w) {
        assert!(tail_ok(&w[0], 31) && tail_ok(&w[4], 24));
        assert!(state == w[0][31]);
        let mut low = [0u8; 8];
        low.copy_from_slice(&w[4][24..]);
        assert!(at == u64::from_be_bytes(low));
    } else {
        assert!(!(tail_ok(&w[0], 31) && tail_ok(&w[4], 24)));
    }
}

#[kani::proof]
#[kani::unwind(33)]
fn state_and_extra_and_balance_guard_their_high_bytes() {
    let w: [[u8; 32]; 5] = kani::any();
    if let Some((state, _)) = state_and_extra(&w) {
        assert!(tail_ok(&w[0], 31) && tail_ok(&w[2], 16) && state == w[0][31]);
    }
    let one: [u8; 32] = kani::any();
    assert!(balance_of(&[one]).is_some() == tail_ok(&one, 16));
}
