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

//! The parser on its own: what it reports for each kind of sequence, and
//! that malformed input ends in a known state.

#[path = "support/log.rs"]
mod log;

use log::run;

#[test]
fn private_marker_and_params() {
    assert_eq!(run(b"\x1b[?1049h"), ["csi?h[1049]"]);
    assert_eq!(run(b"\x1b[>c"), ["csi>c"]);
    assert_eq!(run(b"\x1b[1;5H"), ["csi\0H[1][5]"]);
}

#[test]
fn colon_sub_parameters_group() {
    assert_eq!(run(b"\x1b[38:2::1:2:3m"), ["csi\0m[38, 2, 0, 1, 2, 3]"]);
    assert_eq!(run(b"\x1b[4:3m"), ["csi\0m[4, 3]"]);
}

#[test]
fn escape_inside_a_sequence_starts_a_new_one() {
    assert_eq!(run(b"\x1b[12\x1b[Aa"), ["csi\0A", "pa"]);
}

#[test]
fn cancel_abandons_a_sequence() {
    assert_eq!(run(b"\x1b[12\x18A"), ["x18", "pA"]);
}

#[test]
fn a_misplaced_marker_ignores_the_sequence() {
    assert_eq!(run(b"\x1b[1?2hz"), ["pz"]);
}

#[test]
fn too_many_params_ignores_the_sequence() {
    let mut s = b"\x1b[".to_vec();
    for _ in 0..40 {
        s.extend_from_slice(b"1;");
    }
    s.extend_from_slice(b"mz");
    assert_eq!(run(&s), ["pz"]);
}
