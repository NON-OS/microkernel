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

//! orderOf read for the dispute's window: five words, the state
//! and deliveredAt each a small number, and nothing else.

use nonos_nox_config::DISPUTE_WINDOW;

use super::dispute_fixture::AT;
use super::fixture::{buy, made, offer, NOW, REFUND_PAIR};
use crate::order::order_of::{order_of, window_end};

fn word(n: u64) -> [u8; 32] {
    let mut w = [0; 32];
    w[24..].copy_from_slice(&n.to_be_bytes());
    w
}

#[test]
fn order_of_reads_state_and_delivered_at_and_the_window_is_the_terms() {
    assert_eq!(order_of(&[word(2), word(9), word(0), word(5), word(AT)]), Some((2, AT)));
    assert_eq!(order_of(&[word(2); 4]), None, "four words");
    assert_eq!(order_of(&[[0xff; 32], word(0), word(0), word(0), word(0)]), None);
    assert_eq!(order_of(&[word(300), word(0), word(0), word(0), word(0)]), None, "no u8");
    let m = made(&buy().order_new(&offer(), REFUND_PAIR, NOW));
    assert_eq!(window_end(&m, AT), Some(AT + DISPUTE_WINDOW));
    assert_eq!(window_end(&m, u64::MAX), None);
}
