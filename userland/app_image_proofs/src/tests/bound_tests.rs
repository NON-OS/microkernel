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

//! Which member of an image carries the program a policy names: one with a
//! file whose last copy is an ELF of exactly those bytes, at any path.

use crate::install::image::bound::carries;

fn h(b: &[u8]) -> [u8; 32] {
    *blake3::hash(b).as_bytes()
}

const SOFFICE: &[u8] = b"\x7fELF soffice.bin";

#[test]
fn a_member_with_the_programs_bytes_carries_it_at_any_path() {
    let m: &[(&[u8], &[u8])] = &[(b"usr/bin/soffice", b"#!/bin/sh"), (b"usr/lib/libreoffice/program/soffice.bin", SOFFICE)];
    assert!(carries(m, &h(SOFFICE), h));
    assert!(!carries(m, &h(b"\x7fELF other"), h));
}

#[test]
fn only_the_last_copy_of_a_name_and_only_an_elf_counts() {
    let replaced: &[(&[u8], &[u8])] = &[(b"usr/lib/p", SOFFICE), (b"usr/lib/p", b"\x7fELF later")];
    assert!(!carries(replaced, &h(SOFFICE), h));
    assert!(carries(replaced, &h(b"\x7fELF later"), h));
    let script: &[(&[u8], &[u8])] = &[(b"usr/lib/s", b"#!/bin/sh")];
    assert!(!carries(script, &h(b"#!/bin/sh"), h));
    assert!(!carries(&[], &h(SOFFICE), h));
}
