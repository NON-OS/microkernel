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

//! The walk's bounds on structure, each met and passed by one: collection
//! depth, the global stack, and report IDs.

use super::build::{app, input, MOUSE_APP};
use crate::descriptors::report::parse;

#[test]
fn collections_nest_eight_deep_and_must_all_close() {
    let nest = |n: usize| [vec![[0xa1, 0x00]; n].concat(), vec![0xc0; n]].concat();
    assert!(parse(&nest(8)).is_ok());
    assert!(parse(&nest(9)).is_err(), "nine deep");
    assert!(parse(&[0xc0]).is_err(), "closing what never opened");
    assert!(parse(MOUSE_APP).is_err(), "left open");
}

#[test]
fn the_global_stack_holds_four() {
    let push_pop = |n: usize| [vec![0xa4; n], vec![0xb4; n]].concat();
    assert!(parse(&push_pop(4)).is_ok());
    assert!(parse(&push_pop(5)).is_err(), "five pushed");
    assert!(parse(&[0xb4]).is_err(), "a pop with nothing pushed");
}

#[test]
fn report_ids_are_one_to_255_eight_of_them_numbering_every_report() {
    let with_ids = |ids: &[u8]| {
        let items: Vec<Vec<u8>> =
            ids.iter().map(|&id| [&[0x85, id][..], &input(8, 1)].concat()).collect();
        app(&items.iter().map(Vec::as_slice).collect::<Vec<_>>())
    };
    assert!(parse(&with_ids(&[1, 2, 3, 4, 5, 6, 7, 255])).is_ok());
    assert!(parse(&with_ids(&[1, 2, 3, 4, 5, 6, 7, 8, 9])).is_err(), "nine reports");
    assert!(parse(&with_ids(&[0])).is_err(), "report ID 0 is reserved");
    assert!(parse(&[0x85, 0x00]).is_err(), "report ID 0, even naming no report");
    let unnumbered = app(&[&input(8, 1), &[0x85, 1], &input(8, 1)]);
    assert!(parse(&unnumbered).is_err(), "a report before any report ID");
}
