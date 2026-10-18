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

//! OUTPUT_ARRANGEMENT's answer: eight u32s in their order, a chosen scale
//! of the panel's own rule written 0, and nothing but these numbers.

use crate::display::arrange_wire::{encode, Arrangement, ARRANGEMENT_LEN};

fn words(b: &[u8; ARRANGEMENT_LEN]) -> Vec<u32> {
    b.chunks(4).map(|c| u32::from_le_bytes(c.try_into().unwrap())).collect()
}

#[test]
fn the_answer_is_eight_words_in_order() {
    let a = Arrangement {
        outputs: 2,
        extend: true,
        extended: true,
        scale_q: 6,
        chosen: [Some(6), None, Some(8), None],
    };
    assert_eq!(words(&encode(&a)), [2, 1, 1, 6, 6, 0, 8, 0]);
}

#[test]
fn extend_chosen_with_one_display_says_so_and_shows_mirrored() {
    let a = Arrangement { outputs: 1, extend: true, scale_q: 4, ..Arrangement::default() };
    assert_eq!(words(&encode(&a))[..4], [1, 1, 0, 4], "chosen, waiting for a second display");
}
