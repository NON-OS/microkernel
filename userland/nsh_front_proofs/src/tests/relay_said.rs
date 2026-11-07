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

//! A relay to nsh.attest that fails is said on the serial console: which
//! call, how it failed, and how long it took, since the client hears BUSY
//! alone. The fault the server sees is the one it saw before.

use crate::relay_said::{line, Fault, ST_ATTEST_REFUSED, ST_REFUSED_CALLER};

const ATTEST: &str = include_str!("../../../capsule_nsh_front/src/attest.rs");

fn said(op: &str, fault: Fault, ms: u64) -> String {
    String::from_utf8(line(op, fault, ms)).expect("the line is text")
}

#[test]
fn each_fault_names_the_call_how_and_how_long() {
    assert_eq!(
        said("doc", Fault::NoAnswer(-110), 5003),
        "[NSH-FRONT] attest relay doc: no answer from nsh.attest, kernel errno -110, after 5003 ms\n"
    );
    assert_eq!(
        said("entries", Fault::Status(ST_REFUSED_CALLER), 0),
        "[NSH-FRONT] attest relay entries: nsh.attest refused this caller, after 0 ms\n"
    );
    assert_eq!(
        said("doc", Fault::Status(ST_ATTEST_REFUSED), 12),
        "[NSH-FRONT] attest relay doc: the kernel would not attest, after 12 ms\n"
    );
    assert_eq!(
        said("doc", Fault::Status(4), 7),
        "[NSH-FRONT] attest relay doc: nsh.attest answered status 4, after 7 ms\n"
    );
    assert_eq!(
        said("entries", Fault::Unframed, 1),
        "[NSH-FRONT] attest relay entries: the answer was not a whole frame, after 1 ms\n"
    );
}

#[test]
fn extreme_numbers_are_written_whole() {
    let s = said("doc", Fault::NoAnswer(i64::MIN), u64::MAX);
    assert!(s.contains("errno -9223372036854775808,"), "{s}");
    assert!(s.ends_with("after 18446744073709551615 ms\n"), "{s}");
    let s = said("doc", Fault::Status(u16::MAX), 0);
    assert!(s.contains("status 65535,"), "{s}");
}

#[test]
fn the_statuses_are_the_wires_own() {
    let wire = include_str!("../../../nsh_front_wire/src/lib.rs");
    assert!(wire.contains(&format!("pub const ST_REFUSED_CALLER: u16 = {ST_REFUSED_CALLER};")));
    assert!(wire.contains(&format!("pub const ST_ATTEST_REFUSED: u16 = {ST_ATTEST_REFUSED};")));
}

#[test]
fn every_relay_goes_through_the_saying_call() {
    assert!(ATTEST.contains("self.call(OP_DOC, challenge)"));
    assert!(ATTEST.contains("self.call(OP_ENTRIES, &[])"));
    assert_eq!(ATTEST.matches("self.relay(").count(), 1, "only call() relays");
    assert!(ATTEST.contains("let _ = mk_debug(text.as_ptr(), text.len());"));
}

#[test]
fn the_server_sees_the_faults_it_saw_before() {
    assert!(ATTEST.contains("Fault::NoAnswer(n) => (AttestFault::Kernel(n), f),"));
    let refused = ATTEST.find("=> (AttestFault::Kernel(REFUSED), f),").expect("the refusals");
    let statuses = &ATTEST[ATTEST.find("Fault::Status(\n").expect("the statuses")..refused];
    for st in ["ST_ATTEST_REFUSED", "ST_REFUSED_CALLER", "ST_NO_TPM"] {
        assert!(statuses.contains(st), "{st} is a refusal");
    }
    assert!(ATTEST.contains("Fault::Status(_) | Fault::Unframed => (AttestFault::Malformed, f),"));
}
