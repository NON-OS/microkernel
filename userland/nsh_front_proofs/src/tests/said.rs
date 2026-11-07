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

//! nsh.attest's serial lines: one per event, each naming the host key id
//! where there is one, so the operator can compare it with what nsh-enrol
//! is given.

use crate::said::{hex, line, Event};

const EVENTS: [Event; 10] = [
    Event::Made,
    Event::MadeUnkept,
    Event::Opened,
    Event::Kept,
    Event::Replaced,
    Event::NoTpm,
    Event::Tpm(-13),
    Event::Store,
    Event::Entropy,
    Event::Request,
];

#[test]
fn every_line_is_one_tagged_line() {
    for e in EVENTS {
        let l = String::from_utf8(line(e, None)).expect("ascii");
        assert!(l.starts_with("[NSH-ATTEST] "), "{e:?}");
        assert_eq!(l.matches('\n').count(), 1, "{e:?}");
        assert!(l.ends_with('\n'));
    }
}

#[test]
fn the_key_id_is_said_in_full() {
    let id: [u8; 32] = core::array::from_fn(|i| i as u8 * 7);
    let l = String::from_utf8(line(Event::Made, Some(&id))).expect("ascii");
    let mut want = Vec::new();
    hex(&mut want, &id);
    let want = String::from_utf8(want).expect("hex");
    assert_eq!(want.len(), 64);
    assert!(l.contains(&format!(", key id {want}\n")), "{l}");
    assert!(l.contains("made on this machine at first boot"));
}

#[test]
fn a_tpm_fault_says_its_errno() {
    let l = String::from_utf8(line(Event::Tpm(-13), None)).expect("ascii");
    assert!(l.ends_with("errno -13\n"), "{l}");
    let l = String::from_utf8(line(Event::NoTpm, None)).expect("ascii");
    assert!(l.contains("no TPM"));
}

#[test]
fn hex_is_lowercase_and_two_digits_a_byte() {
    let mut out = Vec::new();
    hex(&mut out, &[0x00, 0x0f, 0xa0, 0xff]);
    assert_eq!(out, b"000fa0ff");
}
