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


//! A delivery's bad-chunk answer is kept and answered again (v1.4.49).

use nonos_nox_meter::orders::{KeptDelivered, MAX_KEPT};

#[test]
fn a_kept_answer_is_given_again_and_replaced_not_doubled() {
    let mut k = KeptDelivered::default();
    assert_eq!(k.get(&[1; 32]), None, "nothing kept: the delivery is opened");
    k.keep([1; 32], vec![1, 2]);
    assert_eq!(k.get(&[1; 32]), Some(&[1u8, 2][..]));
    k.keep([1; 32], vec![3]);
    assert_eq!(k.get(&[1; 32]), Some(&[3u8][..]));
    assert_eq!(k.get(&[2; 32]), None);
}

#[test]
fn past_max_kept_the_oldest_is_dropped() {
    let mut k = KeptDelivered::default();
    for i in 0..=MAX_KEPT as u8 {
        k.keep([i; 32], vec![i]);
    }
    assert_eq!(k.get(&[0; 32]), None, "the oldest went first");
    assert_eq!(k.get(&[1; 32]), Some(&[1u8][..]));
    assert_eq!(k.get(&[MAX_KEPT as u8; 32]), Some(&[MAX_KEPT as u8][..]));
}
