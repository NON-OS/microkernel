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

//! A SERVE_ONE listen the kernel refuses (net/serve_one/refusal.rs): EPERM,
//! a run the kernel does not pair with the capsule, is said as that, not
//! as a policy without SERVE_ONE, which the bound policy did grant (seen
//! for SimpleX's core on a development guest, its listing under that
//! image's own market key).

use crate::serve_refusal::{not_paired, of, Refusal};

#[test]
fn each_kernel_answer_is_its_refusal() {
    assert_eq!(of(-16), Refusal::InUse);
    assert_eq!(of(-12), Refusal::NoRoom);
    assert_eq!(of(-1), Refusal::NotPaired);
    for rc in [-22, -14, 0, i64::MIN, -2, -13] {
        assert_eq!(of(rc), Refusal::Refused, "{rc}");
    }
}

#[test]
fn the_not_paired_line_names_the_capsule_and_never_a_missing_grant() {
    let said = not_paired("app.simplex");
    assert!(said
        .starts_with("[LINUX] refused listen: the kernel serves app.simplex only to a store run"));
    assert!(!said.contains("does not grant"));
    assert!(said.ends_with('\n'));
}

#[test]
fn a_control_byte_in_the_name_is_shown_not_obeyed() {
    let said = not_paired("app\n.x\u{1b}[2J");
    assert!(said.contains("app?.x?[2J"));
    assert_eq!(said.matches('\n').count(), 1);
}
