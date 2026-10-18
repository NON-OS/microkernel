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

//! The block wire as the driver answers it: who may reach the medium, the
//! units, GEOMETRY, IDENTIFY, FLUSH, and every malformed request.

use nonos_libc::store_write;

use super::util::{ask, ask_from, body, qemu, request, rw, status, up, SENDER};
use crate::model::Drive;
use crate::server::endpoint::REPLY_ENDPOINT;
use crate::wire::{HDR_LEN, OP_FLUSH, OP_GEOMETRY, OP_HEALTHCHECK, OP_IDENTIFY, OP_READ, OP_WRITE};

#[test]
fn the_reply_echoes_op_flags_unit_and_id() {
    let (mut d, _) = up(qemu(64));
    let r = ask(&mut d, &request(OP_HEALTHCHECK, 0, 0xDEAD_BEEF, &[]));
    assert_eq!(&r[0..4], &0x4e42_5731u32.to_le_bytes());
    assert_eq!(u16::from_le_bytes([r[6], r[7]]), OP_HEALTHCHECK);
    assert_eq!(u16::from_le_bytes([r[8], r[9]]), 3);
    assert_eq!(u16::from_le_bytes([r[10], r[11]]), 0);
    assert_eq!(u32::from_le_bytes(r[12..16].try_into().unwrap()), 0xDEAD_BEEF);
    assert_eq!(status(&r), 0);
    assert_eq!(REPLY_ENDPOINT, 4_294_967_355);
}

#[test]
fn every_op_on_a_unit_with_no_disk_is_enxio() {
    let (mut d, _) = up(qemu(64));
    for unit in [1u16, 2, 3, 4, 0xFFFF] {
        for op in [OP_HEALTHCHECK, OP_GEOMETRY, OP_READ, OP_WRITE, OP_FLUSH, OP_IDENTIFY] {
            let r = ask(&mut d, &request(op, unit, 1, &[]));
            assert_eq!(status(&r), -6, "op {op} unit {unit}");
            assert_eq!(u16::from_le_bytes([r[10], r[11]]), unit);
        }
    }
}

#[test]
fn the_medium_is_the_kernels_and_a_store_write_holders_alone() {
    let (mut d, p) = up(qemu(64));
    store_write(false);
    for op in [OP_GEOMETRY, OP_READ, OP_WRITE, OP_FLUSH, OP_IDENTIFY] {
        let r = ask_from(&mut d, SENDER, &request(op, 0, 1, &rw(0, 1, &[])));
        assert_eq!(status(&r), -13, "op {op}");
    }
    assert_eq!(p.borrow().ch[0].log.len(), 1, "only bring-up's IDENTIFY reached the disk");
    let r = ask_from(&mut d, SENDER, &request(OP_HEALTHCHECK, 0, 1, &[]));
    assert_eq!(status(&r), 0, "a health check stays open");
    store_write(true);
    let r = ask_from(&mut d, SENDER, &request(OP_READ, 0, 1, &rw(0, 1, &[])));
    assert_eq!(status(&r), 0);
}

#[test]
fn geometry_names_the_capacity_the_sector_the_largest_transfer_and_a_fixed_disk() {
    let (mut d, _) = up(qemu(0x1_0000_0123));
    let r = ask(&mut d, &request(OP_GEOMETRY, 0, 1, &[]));
    assert_eq!(status(&r), 0);
    let b = body(&r);
    assert_eq!(b.len(), 20);
    assert_eq!(u64::from_le_bytes(b[0..8].try_into().unwrap()), 0x1_0000_0123);
    assert_eq!(u32::from_le_bytes(b[8..12].try_into().unwrap()), 512);
    assert_eq!(u32::from_le_bytes(b[12..16].try_into().unwrap()), 65536);
    assert_eq!(
        u32::from_le_bytes(b[16..20].try_into().unwrap()),
        4,
        "present, not removable, writable"
    );
}

#[test]
fn identify_gives_the_model_the_serial_and_ata() {
    let mut disk = Drive::disk(64, true);
    crate::model::put_text(&mut disk.identify[10..20], b"   VB1234-5678      ");
    let (mut d, _) = up([[Some(disk), None], [None, None]]);
    let r = ask(&mut d, &request(OP_IDENTIFY, 0, 1, &[]));
    assert_eq!(status(&r), 0);
    let b = body(&r);
    assert_eq!(b.len(), 64);
    // Lengths, ATA, reserved; the serial without its padding either side.
    assert_eq!((b[0], b[1], b[2], b[3]), (13, 11, 5, 0));
    assert_eq!(&b[4..17], b"QEMU HARDDISK");
    assert!(b[17..44].iter().all(|&c| c == 0));
    assert_eq!(&b[44..55], b"VB1234-5678");
    assert!(b[55..].iter().all(|&c| c == 0));
}

#[test]
fn identify_of_a_model_that_is_not_text_is_enodev() {
    let mut disk = Drive::disk(64, true);
    disk.identify[27] = 0x0141;
    let (mut d, _) = up([[Some(disk), None], [None, None]]);
    let r = ask(&mut d, &request(OP_IDENTIFY, 0, 1, &[]));
    assert_eq!(status(&r), -19);
}

#[test]
fn flush_issues_the_command_the_disk_takes() {
    let (mut d, p) =
        up([[Some(Drive::disk(64, true)), Some(Drive::disk(64, false))], [None, None]]);
    assert_eq!(status(&ask(&mut d, &request(OP_FLUSH, 0, 1, &[]))), 0);
    assert_eq!(p.borrow().ch[0].log.last().unwrap().0, 0xEA);
    assert_eq!(status(&ask(&mut d, &request(OP_FLUSH, 1, 1, &[]))), 0);
    assert_eq!(p.borrow().ch[0].log.last().unwrap(), &(0xE7, 1, 0, 256));
    assert_eq!(p.borrow().ch[0].drives[1].as_ref().unwrap().flushes, 1);
}

#[test]
fn flush_of_a_disk_with_no_cache_sends_nothing() {
    let mut disk = Drive::disk(64, false);
    disk.identify[82] = 0x4000;
    disk.identify[83] = 0x4000;
    let (mut d, p) = up([[Some(disk), None], [None, None]]);
    let before = p.borrow().ch[0].log.len();
    assert_eq!(status(&ask(&mut d, &request(OP_FLUSH, 0, 1, &[]))), 0);
    assert_eq!(p.borrow().ch[0].log.len(), before);
}

#[test]
fn malformed_requests() {
    let (mut d, p) = up(qemu(64));
    let cases: Vec<(Vec<u8>, i32)> = vec![
        (request(OP_GEOMETRY, 0, 1, &[0]), -22),
        (request(OP_HEALTHCHECK, 0, 1, &[0; 4]), -22),
        (request(99, 0, 1, &[]), -22),
        (request(OP_READ, 0, 1, &rw(0, 0, &[])), -22),
        (request(OP_READ, 0, 1, &rw(0, 129, &[])), -22),
        (request(OP_READ, 0, 1, &rw(64, 1, &[])), -6),
        (request(OP_READ, 0, 1, &rw(63, 2, &[])), -6),
        (request(OP_READ, 0, 1, &rw(u64::MAX, 1, &[])), -6),
        (request(OP_READ, 0, 1, &rw(0, 1, &[0])), -90),
        (request(OP_READ, 0, 1, &[0; 11]), -90),
        (request(OP_WRITE, 0, 1, &rw(0, 1, &[0; 511])), -90),
        (request(OP_WRITE, 0, 1, &rw(0, 1, &[0; 513])), -90),
        (request(OP_WRITE, 0, 1, &rw(0, 2, &[0; 512])), -90),
    ];
    for (msg, want) in cases {
        assert_eq!(status(&ask(&mut d, &msg)), want, "{msg:?}");
    }
    // A header whose length is not what follows it.
    let mut m = request(OP_READ, 0, 1, &rw(0, 1, &[]));
    m[16] = 13;
    assert_eq!(status(&ask(&mut d, &m)), -90);
    assert_eq!(p.borrow().ch[0].log.len(), 1, "none reached the disk");
}

#[test]
fn a_frame_that_is_not_the_wire_is_refused_under_a_zero_header() {
    let (mut d, _) = up(qemu(64));
    for msg in [
        vec![0u8; 8],
        {
            let mut m = request(OP_READ, 0, 7, &[]);
            m[0] ^= 1;
            m
        },
        {
            let mut m = request(OP_READ, 0, 7, &[]);
            m[4] = 2;
            m
        },
    ] {
        let r = ask(&mut d, &msg);
        assert_eq!(status(&r), -22);
        assert_eq!(&r[6..HDR_LEN - 4], &[0u8; 10], "op, flags, unit, id all zero");
    }
}
