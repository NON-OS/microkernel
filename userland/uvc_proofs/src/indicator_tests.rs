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
//! The compositor's indicator: the notice's bytes, and START refused
//! unless the reply is a whole NCMP reply with status 0 (Decision 9: no
//! indicator, no stream).

use crate::indicator::{notice, shown};

fn reply(op: u16, status: i32) -> [u8; 32] {
    let mut r = [0u8; 32];
    r[0..4].copy_from_slice(&0x4E43_4D50u32.to_le_bytes());
    r[4..6].copy_from_slice(&1u16.to_le_bytes());
    r[6..8].copy_from_slice(&op.to_le_bytes());
    r[16..20].copy_from_slice(&4u32.to_le_bytes());
    r[20..24].copy_from_slice(&status.to_le_bytes());
    r
}

#[test]
fn the_notice_names_the_camera_and_the_state() {
    let on = notice(true);
    assert_eq!(&on[0..4], b"PMCN", "NCMP little-endian");
    assert_eq!(&on[4..8], &[1, 0, 0x0A, 0]);
    assert_eq!(&on[16..20], &8u32.to_le_bytes());
    assert_eq!(&on[20..28], &[1, 0, 0, 0, 1, 0, 0, 0]);
    assert_eq!(&notice(false)[24..28], &[0; 4]);
}

#[test]
fn only_status_zero_shows_the_indicator() {
    assert!(shown(24, &reply(0x000A, 0)));
    assert!(!shown(24, &reply(0x000A, -22)), "a compositor without the op");
    assert!(!shown(-19, &reply(0x000A, 0)), "no compositor");
    assert!(!shown(20, &reply(0x000A, 0)), "no status word");
    assert!(!shown(24, &reply(0x0009, 0)), "another op's reply");
    let mut other = reply(0x000A, 0);
    other[0] = 0;
    assert!(!shown(24, &other), "not NCMP");
}
