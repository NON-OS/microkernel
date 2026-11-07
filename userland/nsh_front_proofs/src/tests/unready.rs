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

//! A host that cannot serve tells the client why in plaintext, with a reason
//! nsh/1 allows before keys, and says the whole reason on the serial line.

use crate::unready::Unready;
use nsh_wire::{plain_abort, Reason};

type U = Unready<&'static str>;

fn all() -> [U; 6] {
    [
        U::NoTpm,
        U::NoAnswer,
        U::Missing { enrol: true, table: false, anchor: true, list: false },
        U::Enrol,
        U::Table("Signature"),
        U::Older,
    ]
}

#[test]
fn each_reason_is_one_a_host_may_tell_before_keys() {
    for u in all() {
        let code = u.reason().code().to_le_bytes();
        assert_eq!(plain_abort(&code), Ok(u.reason()), "{u:?}");
    }
}

#[test]
fn no_tpm_is_unattested_a_passing_fault_is_busy_and_the_rest_unenrolled() {
    assert_eq!(U::NoTpm.reason(), Reason::Unattested);
    assert_eq!(U::NoAnswer.reason(), Reason::Busy);
    for u in &all()[2..] {
        assert_eq!(u.reason(), Reason::Unenrolled, "{u:?}");
    }
}

#[test]
fn the_line_names_each_record_the_machine_lacks() {
    let l = String::from_utf8(all()[2].line()).expect("ascii");
    assert_eq!(
        l,
        "[NSH-FRONT] turned away: not enrolled, /nonos/nsh holds no enrol roles.anchor\n"
    );
    let every = U::Missing { enrol: true, table: true, anchor: true, list: true };
    let l = String::from_utf8(every.line()).expect("ascii");
    assert!(l.ends_with("holds no enrol roles roles.anchor revocations\n"), "{l}");
}

#[test]
fn the_line_carries_the_tables_fault() {
    let l = String::from_utf8(U::Table("Signature").line()).expect("ascii");
    assert!(l.contains("does not open: \"Signature\""), "{l}");
    for u in all() {
        let l = String::from_utf8(u.line()).expect("ascii");
        assert!(l.starts_with("[NSH-FRONT] turned away: ") && l.ends_with('\n'));
    }
}
