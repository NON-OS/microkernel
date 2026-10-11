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

//! Requests through the mailboxes: what each CCB carries, the bytes moved,
//! and every completion.

use crate::model::sim::Unit;
use crate::wire::{OP_FLUSH, OP_READ, OP_WRITE};

use super::util::{ask, body, request, rw, status, up};

#[test]
fn a_read_ccb_carries_its_buffer_sense_target_and_lun() {
    let mut t4 = Unit::disk(64, 512);
    t4.luns = vec![0, 5];
    let (mut d, m) = up(vec![((4, 0), t4), ((4, 5), Unit::disk(64, 512))]);
    m.borrow_mut().units.get_mut(&(4, 5)).unwrap().store.insert(6, vec![0x42; 512]);
    let r = ask(&mut d, &request(OP_READ, 1, &rw(6, 1, &[])));
    assert_eq!((status(&r), body(&r)), (0, &[0x42u8; 512][..]));
    let s = m.borrow();
    let c = s.ccbs.last().unwrap();
    assert_eq!((c.opcode, c.direction, c.sense_len), (0x03, 1 << 3, 96));
    assert_eq!((c.data, c.data_len, c.sense), (0x0010_1000, 512, 0x0010_0100));
    assert_eq!((c.target, c.lun), (4, 5));
    assert_eq!(c.cdb, vec![0x28, 0, 0, 0, 0, 6, 0, 0, 1, 0]);
    assert_eq!(s.interrupt, 0, "the mailbox interrupt cleared");
}

#[test]
fn a_write_goes_out_and_a_flush_carries_no_data() {
    let (mut d, m) = up(vec![((0, 0), Unit::disk(64, 512))]);
    assert_eq!(status(&ask(&mut d, &request(OP_WRITE, 0, &rw(1, 1, &[0x99; 512])))), 0);
    assert_eq!(m.borrow().units[&(0, 0)].block(1), vec![0x99; 512]);
    assert_eq!(m.borrow().ccbs.last().unwrap().direction, 2 << 3);
    assert_eq!(status(&ask(&mut d, &request(OP_FLUSH, 0, &[]))), 0);
    let s = m.borrow();
    let c = s.ccbs.last().unwrap();
    assert_eq!((c.direction, c.data, c.data_len, c.cdb[0]), (3 << 3, 0, 0, 0x35));
    assert_eq!(s.units[&(0, 0)].flushes, 1);
}

#[test]
fn many_requests_through_one_mailbox_pair() {
    let (mut d, m) = up(vec![((0, 0), Unit::disk(64, 512))]);
    for i in 0..200u64 {
        assert_eq!(status(&ask(&mut d, &request(OP_READ, 0, &rw(i % 64, 1, &[])))), 0);
    }
    assert!(m.borrow().log.iter().filter(|l| l.0 == 0x02).count() > 200);
}

#[test]
fn completions_and_host_statuses() {
    let (mut d, m) = up(vec![((0, 0), Unit::disk(64, 512))]);
    let read = |d: &mut crate::setup::driver::Driver| {
        status(&ask(d, &request(OP_READ, 0, &rw(0, 1, &[]))))
    };
    m.borrow_mut().host = Some(0x11);
    assert_eq!(read(&mut d), -6, "selection timeout");
    for h in [0x13u8, 0x22, 0x23] {
        m.borrow_mut().host = Some(h);
        assert_eq!(read(&mut d), 0, "{h:#x} is tried again");
    }
    for h in [0x14u8, 0x16, 0x1A, 0x20] {
        m.borrow_mut().host = Some(h);
        assert_eq!(read(&mut d), -5, "{h:#x} fails the request");
    }
    for code in [2u8, 3] {
        m.borrow_mut().completion = Some(code);
        assert_eq!(read(&mut d), 0, "an aborted CCB is sent again");
    }
    m.borrow_mut().completion = Some(9);
    assert_eq!(read(&mut d), -5);
    m.borrow_mut().wrong_ccb = true;
    assert_eq!(read(&mut d), -5, "a completion for a CCB the driver did not send");
    assert_eq!(read(&mut d), 0);
}

#[test]
fn sense_residuals_and_an_underrun_status() {
    let (mut d, m) = up(vec![((0, 0), Unit::disk(64, 512))]);
    m.borrow_mut().units.get_mut(&(0, 0)).unwrap().fail_next = Some((0x3, 0x11, 0x00));
    assert_eq!(status(&ask(&mut d, &request(OP_READ, 0, &rw(0, 1, &[])))), -5);
    m.borrow_mut().units.get_mut(&(0, 0)).unwrap().write_protect = true;
    assert_eq!(status(&ask(&mut d, &request(OP_WRITE, 0, &rw(0, 1, &[0; 512])))), -30);
    m.borrow_mut().units.get_mut(&(0, 0)).unwrap().short_read = true;
    assert_eq!(status(&ask(&mut d, &request(OP_READ, 0, &rw(0, 2, &[])))), -5);
    m.borrow_mut().underrun_status = true;
    assert_eq!(
        status(&ask(&mut d, &request(OP_READ, 0, &rw(0, 2, &[])))),
        -5,
        "12h with its residual"
    );
    m.borrow_mut().units.get_mut(&(0, 0)).unwrap().short_read = false;
    assert_eq!(status(&ask(&mut d, &request(OP_READ, 0, &rw(0, 2, &[])))), 0);
}

#[test]
fn a_request_never_completed_resets_the_adapter_and_the_next_works_once() {
    let (mut d, m) = up(vec![((0, 0), Unit::disk(64, 512))]);
    m.borrow_mut().units.get_mut(&(0, 0)).unwrap().store.insert(2, vec![2; 512]);
    m.borrow_mut().stall = true;
    let resets = m.borrow().resets;
    assert_eq!(status(&ask(&mut d, &request(OP_READ, 0, &rw(2, 1, &[])))), -110);
    assert_eq!(m.borrow().resets, resets + 1);
    assert!(m.borrow().mbox.is_some(), "the mailboxes given again");
    let n = m.borrow().ccbs.len();
    let r = ask(&mut d, &request(OP_READ, 0, &rw(2, 1, &[])));
    assert_eq!((status(&r), body(&r)), (0, &[2u8; 512][..]));
    assert_eq!(m.borrow().ccbs.len(), n + 1);
}

#[test]
fn an_adapter_that_does_not_come_back_is_given_up_on() {
    let (mut d, m) = up(vec![((0, 0), Unit::disk(64, 512))]);
    {
        let mut s = m.borrow_mut();
        s.stall = true;
        s.refuse_mailboxes = true;
    }
    assert_eq!(status(&ask(&mut d, &request(OP_READ, 0, &rw(0, 1, &[])))), -110);
    assert!(d.units.hba.dead);
    let n = m.borrow().ccbs.len();
    assert_eq!(status(&ask(&mut d, &request(OP_READ, 0, &rw(0, 1, &[])))), -5);
    assert_eq!(m.borrow().ccbs.len(), n);
}
