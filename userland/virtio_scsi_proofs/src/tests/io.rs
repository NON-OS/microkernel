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

//! The block wire over the adapter: the chains each op sends, the bytes
//! they move, and what each unit answers.

use nonos_libc::store_write;
use nonos_scsi::Hba;

use crate::model::sim::Unit;
use crate::wire::{OP_FLUSH, OP_GEOMETRY, OP_HEALTHCHECK, OP_IDENTIFY, OP_READ, OP_WRITE};

use super::util::{ask, ask_from, body, request, rw, status, up};

#[test]
fn a_read_is_header_response_data_and_returns_the_blocks() {
    let (mut u, m) = up(vec![((0, 0), Unit::disk(4096, 512))]);
    m.unit(0, 0).store.insert(5, vec![0x5A; 512]);
    m.unit(0, 0).store.insert(6, vec![0x6B; 512]);
    let r = ask(&mut u, &request(OP_READ, 0, 1, &rw(5, 2, &[])));
    assert_eq!(status(&r), 0);
    assert_eq!(&body(&r)[..512], &[0x5A; 512][..]);
    assert_eq!(&body(&r)[512..], &[0x6B; 512][..]);
    let rest = m.rest.borrow();
    let c = rest.chains.last().unwrap();
    let va = u.hba.va;
    assert_eq!(
        c.segs,
        vec![(va + 0x1000, 51, false), (va + 0x1100, 108, true), (va + 0x2000, 1024, true)]
    );
    assert_eq!(&c.cdb[..10], &[0x28, 0, 0, 0, 0, 5, 0, 0, 2, 0]);
    assert!(c.cdb[10..].iter().all(|&b| b == 0), "the CDB padded with zeros");
}

#[test]
fn a_write_is_header_data_response_and_lands() {
    let (mut u, m) = up(vec![((0, 0), Unit::disk(4096, 512))]);
    let r = ask(&mut u, &request(OP_WRITE, 0, 2, &rw(9, 1, &[0xC3; 512])));
    assert_eq!(status(&r), 0);
    assert_eq!(m.unit(0, 0).block(9), vec![0xC3; 512]);
    let rest = m.rest.borrow();
    let c = rest.chains.last().unwrap();
    let va = u.hba.va;
    assert_eq!(
        c.segs,
        vec![(va + 0x1000, 51, false), (va + 0x2000, 512, false), (va + 0x1100, 108, true)]
    );
}

#[test]
fn every_request_carries_a_new_id() {
    let (mut u, m) = up(vec![((0, 0), Unit::disk(64, 512))]);
    ask(&mut u, &request(OP_READ, 0, 1, &rw(0, 1, &[])));
    ask(&mut u, &request(OP_READ, 0, 1, &rw(0, 1, &[])));
    let rest = m.rest.borrow();
    let ids: Vec<u64> = rest.chains.iter().map(|c| c.id).collect();
    assert!(ids.windows(2).all(|w| w[1] == w[0] + 1), "{ids:?}");
}

#[test]
fn a_flush_is_header_and_response_alone() {
    let (mut u, m) = up(vec![((0, 0), Unit::disk(64, 512))]);
    assert_eq!(status(&ask(&mut u, &request(OP_FLUSH, 0, 3, &[]))), 0);
    assert_eq!(m.unit(0, 0).flushes, 1);
    assert_eq!(m.rest.borrow().chains.last().unwrap().segs.len(), 2);
}

#[test]
fn geometry_of_a_disk_and_of_a_cd_drive_with_and_without_a_disc() {
    let (mut u, m) = up(vec![((0, 0), Unit::disk(0x1_0000_0001, 4096)), ((1, 0), Unit::cd(300))]);
    let r = ask(&mut u, &request(OP_GEOMETRY, 0, 4, &[]));
    let b = body(&r);
    assert_eq!(u64::from_le_bytes(b[0..8].try_into().unwrap()), 0x1_0000_0001);
    assert_eq!(u32::from_le_bytes(b[8..12].try_into().unwrap()), 4096);
    assert_eq!(u32::from_le_bytes(b[12..16].try_into().unwrap()), 65536);
    assert_eq!(u32::from_le_bytes(b[16..20].try_into().unwrap()), 4, "present");
    let r = ask(&mut u, &request(OP_GEOMETRY, 1, 5, &[]));
    assert_eq!(
        u32::from_le_bytes(body(&r)[16..20].try_into().unwrap()),
        7,
        "removable, read-only, present"
    );
    m.unit(1, 0).medium = false;
    let r = ask(&mut u, &request(OP_GEOMETRY, 1, 6, &[]));
    assert_eq!(u32::from_le_bytes(body(&r)[16..20].try_into().unwrap()), 3, "the disc is out");
    assert_eq!(status(&ask(&mut u, &request(OP_READ, 1, 7, &rw(0, 1, &[])))), -123);
    m.unit(1, 0).medium = true;
    m.unit(1, 0).attentions = 1;
    let r = ask(&mut u, &request(OP_GEOMETRY, 1, 8, &[]));
    assert_eq!(u32::from_le_bytes(body(&r)[16..20].try_into().unwrap()), 7, "and back");
    let r = ask(&mut u, &request(OP_READ, 1, 9, &rw(299, 1, &[])));
    assert_eq!((status(&r), body(&r).len()), (0, 2048));
}

#[test]
fn a_cd_drive_is_never_written_and_flush_is_nothing() {
    let (mut u, m) = up(vec![((1, 0), Unit::cd(300))]);
    let before = m.rest.borrow().chains.len();
    assert_eq!(status(&ask(&mut u, &request(OP_WRITE, 0, 1, &rw(0, 1, &[0; 2048])))), -30);
    assert_eq!(status(&ask(&mut u, &request(OP_FLUSH, 0, 1, &[]))), 0);
    assert_eq!(m.rest.borrow().chains.len(), before, "neither reached the drive");
}

#[test]
fn identify_gives_vendor_and_product_the_serial_and_scsi() {
    let (mut u, _) = up(vec![((0, 0), Unit::disk(64, 512))]);
    let r = ask(&mut u, &request(OP_IDENTIFY, 0, 1, &[]));
    let b = body(&r);
    assert_eq!((b[0], b[1], b[2]), (18, 17, 6));
    assert_eq!(&b[4..22], b"QEMU QEMU HARDDISK");
    assert_eq!(&b[44..61], b"drive-scsi0-0-0-0");
}

#[test]
fn a_unit_with_no_vendor_is_named_by_its_product_alone() {
    let mut d = Unit::disk(64, 512);
    d.vendor = b"";
    let (mut u, _) = up(vec![((0, 0), d)]);
    let r = ask(&mut u, &request(OP_IDENTIFY, 0, 1, &[]));
    assert_eq!(body(&r)[0], 13);
    assert_eq!(&body(&r)[4..17], b"QEMU HARDDISK");
}

#[test]
fn a_request_past_the_buffer_or_the_device_limit_is_refused_before_it_is_sent() {
    let m = crate::model::device(vec![((0, 0), Unit::disk(4096, 512))]);
    m.rest.borrow_mut().cfg.max_sectors = 16;
    let mut u = super::util::up_on(&m);
    assert_eq!(u.hba.buffer().len(), 8192);
    let before = m.rest.borrow().chains.len();
    assert_eq!(status(&ask(&mut u, &request(OP_READ, 0, 1, &rw(0, 17, &[])))), -22);
    assert_eq!(status(&ask(&mut u, &request(OP_READ, 0, 1, &rw(4096, 1, &[])))), -6);
    assert_eq!(m.rest.borrow().chains.len(), before);
    let r = ask(&mut u, &request(OP_GEOMETRY, 0, 1, &[]));
    assert_eq!(u32::from_le_bytes(body(&r)[12..16].try_into().unwrap()), 8192);
}

#[test]
fn malformed_requests_reach_nothing() {
    let (mut u, m) = up(vec![((0, 0), Unit::disk(64, 512))]);
    let before = m.rest.borrow().chains.len();
    let cases: Vec<(Vec<u8>, i32)> = vec![
        (request(OP_GEOMETRY, 0, 1, &[0]), -22),
        (request(OP_FLUSH, 0, 1, &[0; 4]), -22),
        (request(99, 0, 1, &[]), -22),
        (request(OP_READ, 0, 1, &rw(0, 0, &[])), -22),
        (request(OP_READ, 0, 1, &rw(0, 129, &[])), -22),
        (request(OP_READ, 0, 1, &rw(63, 2, &[])), -6),
        (request(OP_READ, 0, 1, &rw(0, 1, &[0])), -90),
        (request(OP_WRITE, 0, 1, &rw(0, 1, &[0; 511])), -90),
        (request(OP_WRITE, 0, 1, &rw(0, 2, &[0; 512])), -90),
    ];
    for (msg, want) in cases {
        assert_eq!(status(&ask(&mut u, &msg)), want, "{msg:?}");
    }
    let mut m2 = request(OP_READ, 0, 1, &rw(0, 1, &[]));
    m2[16] = 13;
    assert_eq!(status(&ask(&mut u, &m2)), -90, "a length that is not what follows");
    m2[0] ^= 1;
    assert_eq!(status(&ask(&mut u, &m2)), -22, "not the wire");
    assert_eq!(m.rest.borrow().chains.len(), before, "none reached the adapter");
}

#[test]
fn units_without_a_disk_and_the_medium_rule() {
    let (mut u, _) = up(vec![((0, 0), Unit::disk(64, 512))]);
    assert_eq!(status(&ask(&mut u, &request(OP_HEALTHCHECK, 1, 1, &[]))), -6);
    store_write(false);
    assert_eq!(status(&ask_from(&mut u, 9, &request(OP_READ, 0, 1, &rw(0, 1, &[])))), -13);
    assert_eq!(status(&ask_from(&mut u, 9, &request(OP_HEALTHCHECK, 0, 1, &[]))), 0);
    store_write(true);
    assert_eq!(status(&ask_from(&mut u, 9, &request(OP_READ, 0, 1, &rw(0, 1, &[])))), 0);
}
