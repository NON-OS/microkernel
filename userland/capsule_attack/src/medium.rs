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

//! Raw sectors without StoreWrite. Each disk driver serves its medium to the
//! kernel's client and to a holder of StoreWrite alone, so a read of the first
//! sector and a write past the end of the disk must both come back EACCES in
//! the driver's own status word. The write aims past the end so that, if the
//! gate ever let it through, the driver's bounds would refuse it instead of a
//! sector being overwritten, and the answer would still show the escape.

use nonos_libc::mk_syscall_raw as raw;

use crate::codes::{MICL, MSVL};
use crate::line::{escaped, note, verdict};

const HDR: usize = 20;
const STATUS: usize = 4;
const RW: usize = 12;
const SECTOR: usize = 512;
const VERSION: u16 = 1;
const EACCES: i64 = -13;
const CALL_MS: u64 = 8000;
/// Past the end of any disk.
const PAST_THE_END: u64 = u64::MAX - 7;

struct Disk {
    service: &'static [u8],
    magic: u32,
    read: u16,
    write: u16,
    read_what: &'static [u8],
    write_what: &'static [u8],
}

/// Each driver's contract, as `nonos_blk_client` copies it.
const DISKS: [Disk; 3] = [
    Disk {
        service: b"driver.nvme0",
        magic: 0x4E4E_564D,
        read: 7,
        write: 8,
        read_what: b"read a raw NVMe sector without StoreWrite, MkIpcCall",
        write_what: b"write a raw NVMe sector without StoreWrite, MkIpcCall",
    },
    Disk {
        service: b"driver.ahci0",
        magic: 0x4E41_4843,
        read: 5,
        write: 6,
        read_what: b"read a raw SATA sector without StoreWrite, MkIpcCall",
        write_what: b"write a raw SATA sector without StoreWrite, MkIpcCall",
    },
    Disk {
        service: b"driver.virtio_blk0",
        magic: 0x4E42_4C4B,
        read: 3,
        write: 4,
        read_what: b"read a raw virtio-blk sector without StoreWrite, MkIpcCall",
        write_what: b"write a raw virtio-blk sector without StoreWrite, MkIpcCall",
    },
];

pub fn raw_disk() {
    let mut tried = false;
    for d in &DISKS {
        let Some(port) = lookup(d.service) else {
            continue;
        };
        tried = true;
        let mut req = [0u8; HDR + RW + SECTOR];
        header(&mut req, d.magic, d.read, RW as u32);
        rw(&mut req[HDR..HDR + RW], 0);
        answer(port, &req[..HDR + RW], d.magic, d.read, d.read_what);
        header(&mut req, d.magic, d.write, (RW + SECTOR) as u32);
        rw(&mut req[HDR..HDR + RW], PAST_THE_END);
        answer(port, &req, d.magic, d.write, d.write_what);
    }
    if !tried {
        note(b"[ATTACK-NOTE] cap-escape: no disk driver is running, raw sectors were not tried\n");
    }
}

fn lookup(name: &[u8]) -> Option<u32> {
    let (mut port, mut pid) = (0u32, 0u32);
    let rc = raw(MSVL, [name.as_ptr() as u64, name.len() as u64, &mut port as *mut u32 as u64, &mut pid as *mut u32 as u64, 0, 0]);
    (rc == 0 && port != 0).then_some(port)
}

fn header(out: &mut [u8], magic: u32, op: u16, payload: u32) {
    out[0..4].copy_from_slice(&magic.to_le_bytes());
    out[4..6].copy_from_slice(&VERSION.to_le_bytes());
    out[6..8].copy_from_slice(&op.to_le_bytes());
    out[8..12].fill(0);
    out[12..16].copy_from_slice(&0x4154_4B21u32.to_le_bytes());
    out[16..20].copy_from_slice(&payload.to_le_bytes());
}

fn rw(out: &mut [u8], lba: u64) {
    out[0..8].copy_from_slice(&lba.to_le_bytes());
    out[8..12].copy_from_slice(&1u32.to_le_bytes());
}

/// The kernel refusing the call is a refusal; so is the driver's EACCES.
/// Any other status means the request reached the medium's handler.
fn answer(port: u32, req: &[u8], magic: u32, op: u16, what: &[u8]) {
    let mut rx = [0u8; HDR + STATUS + SECTOR];
    let rc = raw(MICL, [u64::from(port), req.as_ptr() as u64, req.len() as u64, rx.as_mut_ptr() as u64, rx.len() as u64, CALL_MS]);
    if rc < 0 {
        verdict(b"cap-escape", what, rc);
        return;
    }
    let n = (rc as usize).min(rx.len());
    if n < HDR + STATUS || le32(&rx, 0) != magic || u16::from_le_bytes([rx[6], rx[7]]) != op {
        note(b"[ATTACK-NOTE] cap-escape: a disk driver gave an answer that is not its reply\n");
        return;
    }
    let status = i64::from(le32(&rx, HDR) as i32);
    if status == EACCES {
        verdict(b"cap-escape", what, EACCES);
    } else {
        escaped(b"cap-escape", what, status);
    }
}

fn le32(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([b[at], b[at + 1], b[at + 2], b[at + 3]])
}
