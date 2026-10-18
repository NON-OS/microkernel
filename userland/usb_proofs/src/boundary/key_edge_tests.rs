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

//! Boot keyboard report edges, by name: each report, fed with one key
//! held, and the key events its rule allows.

use crate::keyboard_tests::{report, step};

/// A named keyboard report and the key events it must make.
type KeyCase = (&'static str, Vec<u8>, Vec<(u8, bool)>);

#[test]
fn keyboard_report_edges() {
    const A: u8 = 0x04;
    let held = step([0; 6], &report(0, &[A])).1;
    let lone = |k: u8| report(0, &[k]).to_vec();
    let mut short = report(0, &[]).to_vec();
    short.truncate(7);
    let mut long = report(0, &[]).to_vec();
    long.push(0);
    // (case, report fed with A held, key events it makes)
    let cases: Vec<KeyCase> = vec![
        ("all zero", report(0, &[]).to_vec(), vec![(A, false)]),
        ("A held", lone(A), vec![]),
        ("ErrorRollOver in every slot", report(0, &[1; 6]).to_vec(), vec![]),
        ("ErrorRollOver in one slot", report(0, &[A, 1]).to_vec(), vec![]),
        ("POSTFail", report(0, &[2; 6]).to_vec(), vec![]),
        ("ErrorUndefined", report(0, &[3; 6]).to_vec(), vec![]),
        ("first key 0x04 with B", report(0, &[A, 0x05]).to_vec(), vec![(0x05, true)]),
        ("last main key 0xa4", lone(0xa4), vec![(0xa4, true), (A, false)]),
        ("first reserved 0xa5", lone(0xa5), vec![(A, false)]),
        ("last reserved 0xaf", lone(0xaf), vec![(A, false)]),
        ("first keypad extra 0xb0", lone(0xb0), vec![(0xb0, true), (A, false)]),
        ("last keypad extra 0xdd", lone(0xdd), vec![(0xdd, true), (A, false)]),
        ("reserved 0xde", lone(0xde), vec![(A, false)]),
        ("left control usage 0xe0", lone(0xe0), vec![(A, false)]),
        ("right GUI usage 0xe7", lone(0xe7), vec![(A, false)]),
        ("reserved 0xe8", lone(0xe8), vec![(A, false)]),
        ("0xff", lone(0xff), vec![(A, false)]),
        (
            "one key in all six slots",
            report(0, &[0x05; 6]).to_vec(),
            vec![(0x05, true), (A, false)],
        ),
        ("every modifier bit", report(0xff, &[A]).to_vec(), vec![]),
        ("empty report", vec![], vec![]),
        ("seven bytes", short, vec![]),
        ("nine bytes", long, vec![]),
    ];
    for (name, raw, want) in cases {
        assert_eq!(step(held, &raw).0, want, "{name}");
    }
}
