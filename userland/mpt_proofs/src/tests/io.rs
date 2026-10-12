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

//! Requests through the FIFOs: what each frame carries, context and
//! address replies, the bytes moved, and every IOC status.

use crate::model::sim::Unit;
use crate::wire::{OP_FLUSH, OP_READ, OP_WRITE};

use super::util::{ask, body, request, rw, status, up};

#[test]
fn a_read_frame_carries_target_lun_cdb_sense_and_one_sge() {
    let mut t3 = Unit::disk(64, 512);
    t3.luns = vec![0, 1];
    let (mut u, m) = up(vec![((3, 0), t3), ((3, 1), Unit::disk(64, 512))]);
    m.unit(3, 1).store.insert(9, vec![0x31; 512]);
    let r = ask(&mut u, &request(OP_READ, 1, &rw(9, 1, &[])));
    assert_eq!((status(&r), body(&r)), (0, &[0x31u8; 512][..]));
    let s = m.0.borrow();
    let io = s.ios.last().unwrap();
    assert_eq!((io.target, io.bus, io.lun), (3, 0, [0, 1, 0, 0, 0, 0, 0, 0]));
    assert_eq!((io.control, io.data_len, io.sense_len), (0x0200_0000, 512, 96));
    assert_eq!(
        (io.sense_addr, io.sge_flags_len, io.sge_addr),
        (0x0010_0800, 0xD100_0200, 0x0010_1000)
    );
    assert_eq!(io.cdb, vec![0x28, 0, 0, 0, 0, 9, 0, 0, 1, 0]);
}

#[test]
fn a_write_is_host_to_ioc_and_a_flush_has_an_empty_sge() {
    let (mut u, m) = up(vec![((0, 0), Unit::disk(64, 512))]);
    assert_eq!(status(&ask(&mut u, &request(OP_WRITE, 0, &rw(4, 1, &[0x5C; 512])))), 0);
    assert_eq!(m.unit(0, 0).block(4), vec![0x5C; 512]);
    let io = m.0.borrow().ios.last().unwrap().clone();
    assert_eq!((io.control, io.sge_flags_len), (0x0100_0000, 0xD500_0200));
    assert_eq!(status(&ask(&mut u, &request(OP_FLUSH, 0, &[]))), 0);
    let io = m.0.borrow().ios.last().unwrap().clone();
    assert_eq!((io.control, io.sge_flags_len, io.sge_addr, io.cdb[0]), (0, 0xD100_0000, 0, 0x35));
}

#[test]
fn contexts_count_up_and_reply_frames_come_back() {
    let (mut u, m) = up(vec![((0, 0), Unit::disk(64, 512))]);
    for _ in 0..50 {
        assert_eq!(status(&ask(&mut u, &request(OP_READ, 0, &rw(70 - 64, 1, &[])))), 0);
        m.unit(0, 0).fail_next = Some((0x1, 0x17, 0x00));
    }
    let s = m.0.borrow();
    let ctx: Vec<u32> = s.ios.iter().map(|i| i.context).collect();
    assert!(ctx.windows(2).all(|w| w[1] == w[0] + 1), "{ctx:?}");
    assert_eq!(s.free.len(), 8, "every address reply's frame was given back");
}

#[test]
fn ioc_statuses() {
    let (mut u, m) = up(vec![((0, 0), Unit::disk(64, 512))]);
    let read = |u: &mut super::util::Up| status(&ask(u, &request(OP_READ, 0, &rw(0, 1, &[]))));
    for s in [0x41u16, 0x42, 0x43] {
        m.0.borrow_mut().iocstatus = Some(s);
        assert_eq!(read(&mut u), -6, "{s:#x}: no target");
    }
    for s in [0x02u16, 0x48, 0x4B, 0x4C] {
        m.0.borrow_mut().iocstatus = Some(s);
        assert_eq!(read(&mut u), 0, "{s:#x} is tried again");
    }
    for s in [0x01u16, 0x44, 0x46, 0x47] {
        m.0.borrow_mut().iocstatus = Some(s);
        assert_eq!(read(&mut u), -5, "{s:#x} fails the request");
    }
    m.0.borrow_mut().wrong_context = true;
    assert_eq!(read(&mut u), -5, "a context reply for another request");
    m.0.borrow_mut().wrong_context = true;
    m.unit(0, 0).fail_next = Some((0x1, 0x17, 0x00));
    assert_eq!(read(&mut u), -5, "an address reply for another request");
    assert_eq!(read(&mut u), 0);
    assert_eq!(m.0.borrow().free.len(), 8);
}

#[test]
fn sense_an_underrun_and_a_recovered_error() {
    let (mut u, m) = up(vec![((0, 0), Unit::disk(64, 512))]);
    m.unit(0, 0).fail_next = Some((0x3, 0x11, 0x00));
    assert_eq!(status(&ask(&mut u, &request(OP_READ, 0, &rw(0, 1, &[])))), -5);
    m.unit(0, 0).attentions = 2;
    assert_eq!(status(&ask(&mut u, &request(OP_READ, 0, &rw(0, 1, &[])))), 0);
    m.unit(0, 0).write_protect = true;
    assert_eq!(status(&ask(&mut u, &request(OP_WRITE, 0, &rw(0, 1, &[0; 512])))), -30);
    m.unit(0, 0).short_read = true;
    assert_eq!(status(&ask(&mut u, &request(OP_READ, 0, &rw(0, 2, &[])))), -5);
    m.unit(0, 0).short_read = false;
    m.unit(0, 0).fail_next = Some((0x1, 0x17, 0x00));
    assert_eq!(
        status(&ask(&mut u, &request(OP_READ, 0, &rw(0, 2, &[])))),
        0,
        "recovered, with every byte"
    );
}

#[test]
fn sense_counts_only_where_the_ioc_flags_it() {
    let (mut u, m) = up(vec![((0, 0), Unit::disk(64, 512))]);
    m.0.borrow_mut().sense_unflagged = true;
    m.unit(0, 0).fail_next = Some((0x1, 0x17, 0x00));
    assert_eq!(
        status(&ask(&mut u, &request(OP_READ, 0, &rw(0, 1, &[])))),
        -5,
        "a CHECK CONDITION with no sense to read is an error, whatever the buffer holds"
    );
}

#[test]
fn a_request_never_completed_resets_the_ioc_and_the_next_works_once() {
    let (mut u, m) = up(vec![((0, 0), Unit::disk(64, 512))]);
    m.unit(0, 0).store.insert(5, vec![5; 512]);
    m.0.borrow_mut().stall = true;
    let n = m.0.borrow().log.len();
    assert_eq!(status(&ask(&mut u, &request(OP_READ, 0, &rw(5, 1, &[])))), -110);
    assert_eq!(m.0.borrow().log[n..], [0x40, 0x03, 0x02], "reset, facts, init");
    let sent = m.unit(0, 0).log.len();
    let r = ask(&mut u, &request(OP_READ, 0, &rw(5, 1, &[])));
    assert_eq!((status(&r), body(&r)), (0, &[5u8; 512][..]));
    assert_eq!(m.unit(0, 0).log.len(), sent + 1);
}

#[test]
fn an_ioc_that_does_not_come_back_is_given_up_on() {
    let (mut u, m) = up(vec![((0, 0), Unit::disk(64, 512))]);
    {
        let mut s = m.0.borrow_mut();
        s.stall = true;
        s.stuck = true;
    }
    assert_eq!(status(&ask(&mut u, &request(OP_READ, 0, &rw(0, 1, &[])))), -110);
    assert!(u.hba.dead);
    let n = m.0.borrow().ios.len();
    assert_eq!(status(&ask(&mut u, &request(OP_READ, 0, &rw(0, 1, &[])))), -5);
    assert_eq!(m.0.borrow().ios.len(), n);
}
