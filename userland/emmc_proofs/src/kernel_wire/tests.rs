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

//! driver.sdhci0 against the kernel client that talks to it.

use super::decode::decode;
use super::frame::frame;
use super::header as k;
use crate::emmc::disk::{DATA_BUF_BYTES, MAX_SECTORS, SECTOR_SIZE};
use crate::protocol as c;

#[test]
fn both_ends_frame_the_same_wire() {
    let sent = frame(k::OP_READ, 2, 7, &[0; 12]);
    let req = c::decode_request(&sent).expect("taken");
    assert_eq!((req.op, req.unit, req.request_id, req.payload_len), (c::OP_READ, 2, 7, 12));
    let mut ahci = sent.clone();
    ahci[..4].copy_from_slice(&0x4E41_4843u32.to_le_bytes());
    assert!(c::decode_request(&ahci).is_none());
    // The reply repeats the kernel's magic, version, op, unit and id.
    let mut reply = [0u8; 24];
    c::encode_response_header(&mut reply, &req, 4);
    assert_eq!(reply[..20], frame(k::OP_READ, 2, 7, &[0; 4])[..20]);
    assert_eq!(c::HDR_LEN, 20);
    let ops = [c::OP_HEALTHCHECK, c::OP_GEOMETRY, c::OP_READ, c::OP_WRITE, c::OP_FLUSH];
    assert_eq!(ops, [k::OP_HEALTHCHECK, k::OP_GEOMETRY, k::OP_READ, k::OP_WRITE, k::OP_FLUSH]);
    assert_eq!(c::OP_IDENTIFY, k::OP_IDENTIFY);
    assert_eq!(c::RW_HEADER_LEN, k::RW_HEADER_LEN);
    assert_eq!(c::GEOMETRY_LEN, k::GEOMETRY_LEN);
}

#[test]
fn the_kernel_reads_the_emmc_geometry_as_a_fixed_writable_disk() {
    let max = MAX_SECTORS * SECTOR_SIZE as u32;
    let mut out = [0u8; c::GEOMETRY_LEN];
    c::encode_geometry(&mut out, 122_142_720, SECTOR_SIZE as u32, max, c::MEDIA_PRESENT);
    let g = decode(&out).expect("twenty bytes");
    assert_eq!((g.lba_count, g.lba_bytes, g.max_transfer_bytes), (122_142_720, 512, max));
    assert!(g.media_present() && !g.removable() && !g.read_only());
    // One request fits the capsule's data buffer and the kernel's ceiling.
    assert_eq!(max as usize, DATA_BUF_BYTES);
    assert!(max <= k::MAX_TRANSFER_BYTES);
}
