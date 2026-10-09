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

//! The XID table against Linux rtl_chip_infos (r8169_main.c): a TxConfig
//! value each revision reports maps to that revision, first match wins, and
//! an XID Linux has no row for is refused.

use crate::chip::{lookup, xid_of, Chip, Lookup};

pub(super) fn ver(txconfig: u32, gmii: bool) -> (u8, &'static str) {
    match lookup(xid_of(txconfig), gmii) {
        Lookup::Known(Chip { ver, name, .. }) => (ver.0, name),
        other => panic!("txconfig {txconfig:#x}: {other:?}"),
    }
}

#[test]
fn the_xid_is_txconfig_bits_31_to_20_masked_with_0xfcf() {
    assert_eq!(xid_of(0x5410_0700), 0x541);
    assert_eq!(xid_of(0xFFFF_FFFF), 0xFCF);
    assert_eq!(xid_of(0x0003_0F00), 0x000);
}

#[test]
fn bit_11_is_ignored_by_the_gigabit_rows_and_kept_by_the_8110_rows() {
    assert_eq!(ver(0xD41 << 20, true).0, 46, "0x7cf masks bit 11 out");
    assert_eq!(lookup(0x840, true), Lookup::Unknown, "0xfc8 keeps bit 11 in");
}

#[test]
fn a_10_100_board_turns_the_8168gu_and_8168h_xids_into_8106eus_and_8107e() {
    assert_eq!(ver(0x509 << 20, false), (43, "RTL8106eus"));
    assert_eq!(ver(0x541 << 20, false), (48, "RTL8107e"));
    assert_eq!(ver(0x609 << 20, false).0, 61, "no other row changes");
}

#[test]
fn the_extended_marker_and_unknown_xids_are_not_a_chip() {
    assert_eq!(lookup(0x7c8, true), Lookup::Extended);
    for xid in [0x000, 0x7c0, 0x100 | 0x800, 0x7c1] {
        assert_eq!(lookup(xid, true), Lookup::Unknown, "xid {xid:#x}");
    }
}
