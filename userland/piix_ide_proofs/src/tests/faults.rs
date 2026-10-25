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

//! A disk that reports an error, and one that never finishes.

use super::util::{ask, body, pattern, qemu, request, rw, status, up};
use crate::model::Fault;
use crate::wire::{OP_FLUSH, OP_READ, OP_WRITE};

#[test]
fn a_disk_error_mid_transfer_answers_eio_and_the_next_request_works() {
    let (mut d, p) = up(qemu(64));
    p.borrow_mut().ch[0].drives[0].as_mut().unwrap().fault = Fault::DmaError;
    let r = ask(&mut d, &request(OP_WRITE, 0, 50, &rw(3, 1, &[1; 512])));
    assert_eq!(status(&r), -5);
    assert_eq!(r.len(), 24, "a status and nothing after it");
    {
        let p = p.borrow();
        assert_eq!(p.ch[0].bmic & 1, 0, "the engine was stopped");
        assert_eq!(p.ch[0].bmis & 0x06, 0);
        assert!(p.ch[0].drives[0].as_ref().unwrap().store.is_empty(), "nothing written");
    }
    p.borrow_mut().ch[0].drives[0].as_mut().unwrap().store.insert(3, pattern(3));
    let r = ask(&mut d, &request(OP_READ, 0, 51, &rw(3, 1, &[])));
    assert_eq!(body(&r), &pattern(3)[..]);
}

#[test]
fn a_transfer_the_disk_ends_early_is_eio_though_the_disk_reports_none() {
    let (mut d, p) = up(qemu(64));
    p.borrow_mut().ch[0].drives[0].as_mut().unwrap().fault = Fault::Short;
    let r = ask(&mut d, &request(OP_READ, 0, 54, &rw(0, 4, &[])));
    assert_eq!(status(&r), -5, "the engine still held part of its table");
    assert_eq!(r.len(), 24);
    assert_eq!(p.borrow().ch[0].bmic & 1, 0, "and was stopped");
}

#[test]
fn an_error_the_disk_reports_after_moving_everything_is_eio() {
    let (mut d, p) = up(qemu(64));
    p.borrow_mut().ch[0].drives[0].as_mut().unwrap().fault = Fault::ErrorAtEnd;
    let r = ask(&mut d, &request(OP_READ, 0, 55, &rw(0, 2, &[])));
    assert_eq!(status(&r), -5);
    assert_eq!(r.len(), 24, "no data with an error");
}

#[test]
fn an_engine_still_draining_when_the_disk_is_done_is_waited_for() {
    let (mut d, p) = up(qemu(64));
    p.borrow_mut().ch[0].drain = 3;
    p.borrow_mut().ch[0].drives[0].as_mut().unwrap().store.insert(2, pattern(2));
    let r = ask(&mut d, &request(OP_READ, 0, 56, &rw(2, 1, &[])));
    assert_eq!(status(&r), 0);
    assert_eq!(body(&r), &pattern(2)[..]);
}

#[test]
fn a_request_past_the_disk_the_disk_itself_refuses_is_eio() {
    let (mut d, p) = up(qemu(64));
    // The driver bounds requests by IDENTIFY; a disk that shrank refuses.
    p.borrow_mut().ch[0].drives[0].as_mut().unwrap().sectors = 10;
    let r = ask(&mut d, &request(OP_READ, 0, 52, &rw(20, 1, &[])));
    assert_eq!(status(&r), -5);
}

#[test]
fn a_hung_disk_times_out_and_the_channel_is_reset() {
    let (mut d, p) = up(qemu(64));
    p.borrow_mut().ch[0].drives[0].as_mut().unwrap().fault = Fault::Hang;
    let started = std::time::Instant::now();
    let r = ask(&mut d, &request(OP_FLUSH, 0, 53, &[]));
    assert_eq!(status(&r), -110);
    assert!(started.elapsed().as_millis() >= 10_000);
    let p = p.borrow();
    assert_eq!(p.ch[0].resets, 2, "bring-up's reset, and the recovery");
    assert_eq!(p.ch[0].status[0] & 0x80, 0, "the disk is free again");
}
