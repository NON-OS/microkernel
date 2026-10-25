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
//! The splash's attestation panel says only what the boot recorded: a
//! development image's capsules read "path only", a kernel whose signature
//! was not checked says so, and ATTESTED needs both the signature and the
//! STARK.

use crate::attest_lines::{lines, Chain};

const OK: u32 = 0xFF00_CC66;
const ACCENT: u32 = 0xFF00_D4AA;
const WARN: u32 = 0xFFFF_AA00;

pub(super) fn said(secure_boot: u8, kernel_sig_ok: u8, zk_verified: u8) -> [(String, u32); 4] {
    lines(Some(Chain { secure_boot, kernel_sig_ok, zk_verified }))
        .map(|(t, c)| (String::from_utf8(t.to_vec()).unwrap(), c))
}

pub(super) fn line(text: &str, colour: u32) -> (String, u32) {
    (text.into(), colour)
}

#[test]
fn a_development_image_says_path_only() {
    let s = said(0, 1, 2);
    assert_eq!(s[0], line("[-] bootloader  secure boot off", WARN));
    assert_eq!(s[1], line("[+] kernel      ed25519 verified", OK));
    assert_eq!(s[2], line("[-] capsules    path only (development)", WARN));
    assert_eq!(s[3], line("UNVERIFIED", WARN));
}

#[test]
fn a_sealed_image_says_attested() {
    let s = said(1, 1, 1);
    assert_eq!(s[0], line("[+] bootloader  secure boot verified", OK));
    assert_eq!(s[2], line("[#] capsules    zk attested", ACCENT));
    assert_eq!(s[3], line("ATTESTED", OK));
    assert_eq!(said(0, 1, 1)[3], line("ATTESTED", OK), "secure boot has its own line");
}

#[test]
fn nothing_checked_is_said_as_not_checked() {
    let s = said(0, 0, 0);
    assert_eq!(s[1], line("[!] kernel      signature not verified", WARN));
    assert_eq!(s[2], line("[!] capsules    not attested", WARN));
    assert_eq!(s[3], line("UNVERIFIED", WARN));
    assert_eq!(said(1, 0, 1)[3], line("UNVERIFIED", WARN), "a STARK without the signature");
    assert_eq!(said(1, 1, 2)[3], line("UNVERIFIED", WARN), "the signature without the STARK");
    assert_eq!(said(2, 2, 3)[..3], said(0, 0, 0)[..3], "only 1 is a pass");
}
