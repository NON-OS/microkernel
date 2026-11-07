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
//! What the audio service said when the Sound page asked whether this
//! The Network page's Nym cover line reads net.nym's OP_TIMING_STATUS answer
//! as net.nym writes it (`capsule_net_nym/src/server/handlers/timing_status.rs`
//! and `respond.rs`), and nothing else.

use crate::nym_cover::{read, request, said, NymCover, HDR_LEN, OP_TIMING_STATUS, STATUS_LEN};

fn answer(profile: u8, out: u16, inbound: u16, errno: u16, payload: u32) -> Vec<u8> {
    let mut r = vec![0u8; HDR_LEN + STATUS_LEN];
    r[0..4].copy_from_slice(&0x4E59_4D31u32.to_le_bytes());
    r[4..6].copy_from_slice(&1u16.to_le_bytes());
    r[6..8].copy_from_slice(&OP_TIMING_STATUS.to_le_bytes());
    r[8..10].copy_from_slice(&errno.to_le_bytes());
    r[12..16].copy_from_slice(&1u32.to_le_bytes());
    r[16..20].copy_from_slice(&payload.to_le_bytes());
    r[HDR_LEN + 4] = profile;
    r[HDR_LEN + 16..HDR_LEN + 18].copy_from_slice(&out.to_le_bytes());
    r[HDR_LEN + 18..HDR_LEN + 20].copy_from_slice(&inbound.to_le_bytes());
    r
}

#[test]
fn the_request_is_a_bare_timing_status_header() {
    let r = request();
    assert_eq!(&r[0..4], &0x4E59_4D31u32.to_le_bytes());
    assert_eq!(u16::from_le_bytes([r[6], r[7]]), OP_TIMING_STATUS);
    assert_eq!(&r[16..20], &[0, 0, 0, 0], "no body");
}

#[test]
fn an_answer_reads_as_its_profile_and_ceilings() {
    assert_eq!(read(&answer(0, 7, 23, 0, 20)), NymCover::Rates { profile: 0, out_kib: 7, in_kib: 23 });
    assert_eq!(said(read(&answer(0, 7, 23, 0, 20))).0, "Low rate, about ");
    assert_eq!(said(read(&answer(1, 78, 234, 0, 20))).0, "Nym rate, about ");
}

#[test]
fn a_refused_short_or_foreign_answer_reads_as_no_service() {
    assert_eq!(read(&answer(0, 7, 23, 3, 20)), NymCover::NoService, "an errno");
    assert_eq!(read(&answer(0, 7, 23, 0, 16)), NymCover::NoService, "an older 16-byte status");
    assert_eq!(read(&answer(0, 7, 23, 0, 20)[..30]), NymCover::NoService, "cut short");
    let mut foreign = answer(0, 7, 23, 0, 20);
    foreign[0] ^= 1;
    assert_eq!(read(&foreign), NymCover::NoService);
    assert_eq!(read(&[]), NymCover::NoService);
}
