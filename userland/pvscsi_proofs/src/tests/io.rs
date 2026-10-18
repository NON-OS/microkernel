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

//! Requests through the rings: what each descriptor carries, which
//! doorbell rings, the bytes moved, and every host status.

use nonos_scsi::Hba;

use crate::model::sim::Unit;
use crate::wire::{OP_FLUSH, OP_GEOMETRY, OP_IDENTIFY, OP_READ, OP_WRITE};

use super::util::{ask, body, request, rw, status, up};

#[test]
fn a_read_carries_its_buffer_sense_and_lun_and_rings_the_rw_doorbell() {
    let mut d = Unit::disk(64, 512);
    d.luns = vec![0, 3];
    let (mut u, m) = up(vec![((9, 0), d), ((9, 3), Unit::disk(64, 512))]);
    m.unit(9, 3).store.insert(4, vec![0x77; 512]);
    let r = ask(&mut u, &request(OP_READ, 1, &rw(4, 1, &[])));
    assert_eq!((status(&r), body(&r)), (0, &[0x77u8; 512][..]));
    let s = m.0.borrow();
    let q = s.seen.last().unwrap();
    let va = u.hba.va;
    assert!(q.rw_kick);
    assert_eq!((q.data_addr, q.data_len, q.flags), (va + 0x4000, 512, 0x08));
    assert_eq!((q.sense_addr, q.sense_len), (va + 0x3000, 96));
    assert_eq!(q.cdb, vec![0x28, 0, 0, 0, 0, 4, 0, 0, 1, 0]);
    assert_eq!((q.lun, q.bus, q.target), ([0, 3, 0, 0, 0, 0, 0, 0], 0, 9));
}

#[test]
fn a_write_goes_to_the_device_and_others_ring_the_non_rw_doorbell() {
    let (mut u, m) = up(vec![((0, 0), Unit::disk(64, 512))]);
    assert_eq!(status(&ask(&mut u, &request(OP_WRITE, 0, &rw(2, 1, &[0xEE; 512])))), 0);
    assert_eq!(m.unit(0, 0).block(2), vec![0xEE; 512]);
    let q = m.0.borrow().seen.last().unwrap().clone();
    assert_eq!((q.flags, q.rw_kick), (0x10, true));
    assert_eq!(status(&ask(&mut u, &request(OP_FLUSH, 0, &[]))), 0);
    let q = m.0.borrow().seen.last().unwrap().clone();
    assert_eq!((q.cdb[0], q.flags, q.data_len, q.data_addr, q.rw_kick), (0x35, 0x04, 0, 0, false));
    assert_eq!(m.unit(0, 0).flushes, 1);
}

#[test]
fn a_grant_that_starts_as_garbage_still_carries_clean_descriptors() {
    let m = crate::model::adapter(vec![((0, 0), Unit::disk(64, 512))]);
    let va = super::util::grant();
    // SAFETY: the grant the test just allocated, GRANT_BYTES long.
    unsafe {
        std::ptr::write_bytes(va as *mut u8, 0xA5, crate::constants::layout::GRANT_BYTES as usize)
    };
    crate::adapter::start(&m, va, va).expect("rings");
    let mut u = crate::units::Units::new(
        crate::hba::PvHba::new(m.clone(), va, va),
        super::util::REPLY,
        b"pv",
    );
    u.scan(0, 0);
    assert_eq!(u.count(), 1, "the LUN bytes the driver never writes are zero");
    assert_eq!(status(&ask(&mut u, &request(OP_READ, 0, &rw(0, 1, &[])))), 0);
}

#[test]
fn every_request_has_its_own_context_and_the_rings_wrap() {
    let (mut u, m) = up(vec![((0, 0), Unit::disk(64, 512))]);
    for _ in 0..300 {
        assert_eq!(status(&ask(&mut u, &request(OP_READ, 0, &rw(0, 1, &[])))), 0);
    }
    let s = m.0.borrow();
    let ctx: Vec<u64> = s.seen.iter().map(|q| q.context).collect();
    assert!(ctx.windows(2).all(|w| w[1] == w[0] + 1));
}

#[test]
fn geometry_identify_and_a_full_transfer() {
    let (mut u, m) = up(vec![((1, 0), Unit::disk(1 << 20, 512))]);
    let r = ask(&mut u, &request(OP_GEOMETRY, 0, &[]));
    assert_eq!(u32::from_le_bytes(body(&r)[12..16].try_into().unwrap()), 65536);
    let r = ask(&mut u, &request(OP_IDENTIFY, 0, &[]));
    assert_eq!(&body(&r)[4..22], b"QEMU QEMU HARDDISK");
    for lba in 0..128 {
        m.unit(1, 0).store.insert(lba, vec![lba as u8; 512]);
    }
    let r = ask(&mut u, &request(OP_READ, 0, &rw(0, 128, &[])));
    assert_eq!((status(&r), body(&r).len(), body(&r)[512 * 127]), (0, 65536, 127));
}

#[test]
fn host_statuses() {
    let (mut u, m) = up(vec![((0, 0), Unit::disk(64, 512))]);
    let read = |u: &mut super::util::Up| status(&ask(u, &request(OP_READ, 0, &rw(0, 1, &[]))));
    m.0.borrow_mut().host = Some(0x11);
    assert_eq!(read(&mut u), -6, "selection timeout: no target");
    for h in [0x13u16, 0x22, 0x23, 0x25, 0x26, 0x30] {
        m.0.borrow_mut().host = Some(h);
        assert_eq!(read(&mut u), 0, "{h:#x} is tried again");
    }
    for h in [0x12u16, 0x14, 0x1a, 0x20, 0x27] {
        m.0.borrow_mut().host = Some(h);
        assert_eq!(read(&mut u), -5, "{h:#x} fails the request");
    }
    m.0.borrow_mut().bad_context = true;
    assert_eq!(read(&mut u), -5, "a completion for another request");
    assert_eq!(read(&mut u), 0, "and the next is fine");
}

#[test]
fn sense_and_a_short_transfer_come_back() {
    let (mut u, m) = up(vec![((0, 0), Unit::disk(64, 512))]);
    m.unit(0, 0).fail_next = Some((0x3, 0x11, 0x00));
    assert_eq!(status(&ask(&mut u, &request(OP_READ, 0, &rw(0, 1, &[])))), -5);
    m.unit(0, 0).attentions = 1;
    assert_eq!(status(&ask(&mut u, &request(OP_READ, 0, &rw(0, 1, &[])))), 0);
    m.unit(0, 0).short_read = true;
    assert_eq!(status(&ask(&mut u, &request(OP_READ, 0, &rw(0, 2, &[])))), -5);
    m.unit(0, 0).write_protect = true;
    assert_eq!(status(&ask(&mut u, &request(OP_WRITE, 0, &rw(0, 1, &[0; 512])))), -30);
}

#[test]
fn a_request_never_completed_resets_the_adapter_and_the_next_works_once() {
    let (mut u, m) = up(vec![((0, 0), Unit::disk(64, 512))]);
    m.unit(0, 0).store.insert(3, vec![3; 512]);
    m.0.borrow_mut().stall = true;
    let n = m.0.borrow().commands.len();
    assert_eq!(status(&ask(&mut u, &request(OP_READ, 0, &rw(3, 1, &[])))), -110);
    assert_eq!(m.0.borrow().commands[n..], [1, 3], "reset and new rings");
    let sent = m.unit(0, 0).log.len();
    let r = ask(&mut u, &request(OP_READ, 0, &rw(3, 1, &[])));
    assert_eq!((status(&r), body(&r)), (0, &[3u8; 512][..]));
    assert_eq!(m.unit(0, 0).log.len(), sent + 1, "nothing replayed");
}

#[test]
fn an_adapter_that_does_not_take_its_rings_back_is_given_up_on() {
    let (mut u, m) = up(vec![((0, 0), Unit::disk(64, 512))]);
    {
        let mut s = m.0.borrow_mut();
        s.stall = true;
        s.refuse_rings = true;
    }
    assert_eq!(status(&ask(&mut u, &request(OP_READ, 0, &rw(0, 1, &[])))), -110);
    assert!(u.hba.dead);
    let seen = m.0.borrow().seen.len();
    assert_eq!(status(&ask(&mut u, &request(OP_READ, 0, &rw(0, 1, &[])))), -5);
    assert_eq!(m.0.borrow().seen.len(), seen);
    let _ = u.hba.buffer();
}
