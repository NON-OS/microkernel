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

//! What the device answers besides OK, and a request it never returns.

use crate::common::STATUS_DRIVER_OK;
use crate::model::sim::Unit;
use crate::wire::{OP_FLUSH, OP_READ, OP_WRITE};

use super::util::{ask, body, request, rw, status, up};

#[test]
fn reset_aborted_busy_and_transport_failure_are_tried_again() {
    let (mut u, m) = up(vec![((0, 0), Unit::disk(64, 512))]);
    for code in [4u8, 2, 5, 6] {
        m.rest.borrow_mut().response = Some(code);
        let r = ask(&mut u, &request(OP_READ, 0, 1, &rw(0, 1, &[])));
        assert_eq!(status(&r), 0, "response {code}");
    }
}

#[test]
fn bad_target_and_failures_end_the_request() {
    let (mut u, m) = up(vec![((0, 0), Unit::disk(64, 512))]);
    m.rest.borrow_mut().response = Some(3);
    assert_eq!(status(&ask(&mut u, &request(OP_READ, 0, 1, &rw(0, 1, &[])))), -6);
    for code in [1u8, 7, 8, 9] {
        m.rest.borrow_mut().response = Some(code);
        assert_eq!(
            status(&ask(&mut u, &request(OP_READ, 0, 1, &rw(0, 1, &[])))),
            -5,
            "response {code}"
        );
    }
}

#[test]
fn sense_and_residual_come_through_the_response() {
    let (mut u, m) = up(vec![((0, 0), Unit::disk(64, 512))]);
    m.unit(0, 0).fail_next = Some((0x3, 0x11, 0x00));
    assert_eq!(status(&ask(&mut u, &request(OP_READ, 0, 1, &rw(0, 1, &[])))), -5);
    m.unit(0, 0).attentions = 2;
    assert_eq!(status(&ask(&mut u, &request(OP_READ, 0, 1, &rw(0, 1, &[])))), 0);
    m.unit(0, 0).write_protect = true;
    assert_eq!(status(&ask(&mut u, &request(OP_WRITE, 0, 1, &rw(0, 1, &[0; 512])))), -30);
}

#[test]
fn a_request_never_returned_times_out_and_the_device_is_brought_back() {
    let (mut u, m) = up(vec![((0, 0), Unit::disk(64, 512))]);
    m.unit(0, 0).store.insert(1, vec![9; 512]);
    m.rest.borrow_mut().stall = true;
    let before = m.common.state().status_writes.len();
    let r = ask(&mut u, &request(OP_READ, 0, 1, &rw(1, 1, &[])));
    assert_eq!(status(&r), -110);
    {
        let st = m.common.state();
        assert_eq!(st.status_writes[before], 0, "reset first");
        assert!(st.status & STATUS_DRIVER_OK != 0, "and up again");
        assert_eq!(st.queues[2].enable, 1);
    }
    assert!(!u.hba.dead);
    let sent = m.unit(0, 0).log.len();
    let r = ask(&mut u, &request(OP_READ, 0, 2, &rw(1, 1, &[])));
    assert_eq!(m.unit(0, 0).log.len(), sent + 1, "the read reached the disk once");
    assert_eq!((status(&r), body(&r)), (0, &[9u8; 512][..]), "the fresh ring works");
    // One flush is one command: nothing from the old ring is replayed.
    assert_eq!(status(&ask(&mut u, &request(OP_FLUSH, 0, 3, &[]))), 0);
    assert_eq!(m.unit(0, 0).flushes, 1);
}

#[test]
fn a_short_read_is_an_error_though_the_status_is_good() {
    let (mut u, m) = up(vec![((0, 0), Unit::disk(64, 512))]);
    m.unit(0, 0).short_read = true;
    let r = ask(&mut u, &request(OP_READ, 0, 1, &rw(0, 2, &[])));
    assert_eq!((status(&r), r.len()), (-5, 24));
}

#[test]
fn a_device_that_does_not_come_back_is_given_up_on() {
    let (mut u, m) = up(vec![((0, 0), Unit::disk(64, 512))]);
    m.rest.borrow_mut().stall = true;
    m.common.with(|s| s.stuck_reset = true);
    assert_eq!(status(&ask(&mut u, &request(OP_READ, 0, 1, &rw(1, 1, &[])))), -110);
    assert!(u.hba.dead);
    let before = m.rest.borrow().chains.len();
    assert_eq!(status(&ask(&mut u, &request(OP_READ, 0, 2, &rw(1, 1, &[])))), -5);
    assert_eq!(m.rest.borrow().chains.len(), before, "nothing more is sent");
}
