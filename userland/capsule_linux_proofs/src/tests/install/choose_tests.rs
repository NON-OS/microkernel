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

//! Which program an install records (install/market/choose.rs): by name with
//! no policy, by the bytes a market policy names with one.

use crate::install::market::choose::choose;

const ELF: &[u8] = b"\x7fELF wallet";
const DAEMON: &[u8] = b"\x7fELF daemon";

fn h(b: &[u8]) -> [u8; 32] {
    *blake3::hash(b).as_bytes()
}

fn pick<'a>(files: &[(&'a [u8], &'a [u8])], name: &str, want: Option<&[u8]>) -> Option<&'a [u8]> {
    choose(files, name.as_bytes(), want.map(h).as_ref(), h)
}

#[test]
fn with_no_policy_the_own_name_then_the_first_elf() {
    let f: &[(&[u8], &[u8])] = &[(b"usr/bin/a", DAEMON), (b"usr/bin/monero", b"#!/bin/sh")];
    assert_eq!(pick(f, "monero", None), Some(&b"usr/bin/monero"[..]), "own name, ELF or not");
    assert_eq!(pick(f, "other", None), Some(&b"usr/bin/a"[..]));
    let none: &[(&[u8], &[u8])] = &[(b"usr/share/x", ELF), (b"usr/bin/s", b"#!")];
    assert_eq!(pick(none, "x", None), None);
}

#[test]
fn a_policy_records_the_program_whose_bytes_it_names() {
    let f: &[(&[u8], &[u8])] = &[
        (b"usr/bin/monero-blockchain-ancestry", DAEMON),
        (b"usr/bin/monero-wallet-cli", ELF),
        (b"usr/bin/monerod", DAEMON),
    ];
    assert_eq!(pick(f, "monero", Some(ELF)), Some(&b"usr/bin/monero-wallet-cli"[..]));
    assert_eq!(pick(f, "monero", Some(DAEMON)), Some(&b"usr/bin/monero-blockchain-ancestry"[..]));
    assert_eq!(pick(f, "monero", Some(b"\x7fELF not here")), None, "bind then refuses");
}

#[test]
fn the_own_name_wins_among_equal_bytes_and_the_last_copy_counts() {
    let f: &[(&[u8], &[u8])] = &[(b"usr/bin/a", ELF), (b"usr/bin/tool", ELF)];
    assert_eq!(pick(f, "tool", Some(ELF)), Some(&b"usr/bin/tool"[..]));
    let over: &[(&[u8], &[u8])] =
        &[(b"usr/bin/w", ELF), (b"usr/bin/w", DAEMON), (b"usr/bin/x", ELF)];
    assert_eq!(
        pick(over, "p", Some(ELF)),
        Some(&b"usr/bin/x"[..]),
        "w's bytes are its last copy's"
    );
    let outside: &[(&[u8], &[u8])] = &[(b"usr/lib/w", ELF), (b"usr/bin/s", b"not an ELF")];
    assert_eq!(pick(outside, "p", Some(ELF)), None);
    assert_eq!(pick(outside, "p", Some(b"not an ELF")), None, "only an ELF is a program");
}
