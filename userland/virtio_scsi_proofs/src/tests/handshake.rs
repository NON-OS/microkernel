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

//! The bring-up against the device: the order, the features, the request
//! sizes, the one queue, and every refusal.

use crate::common::{
    STATUS_ACKNOWLEDGE, STATUS_DRIVER, STATUS_DRIVER_OK, STATUS_FAILED, STATUS_FEATURES_OK,
};
use crate::features::VIRTIO_F_VERSION_1;
use crate::model::device;
use crate::model::sim::Unit;
use crate::setup::handshake;

use super::util::{grant, hba, regs, AREA};

#[test]
fn the_handshake_in_order_with_version_1_alone_and_the_request_queue() {
    let m = device(vec![((0, 0), Unit::disk(64, 512))]);
    let (h, l) = hba(&m);
    let st = m.common.state();
    let ack = STATUS_ACKNOWLEDGE;
    let drv = ack | STATUS_DRIVER;
    let fok = drv | STATUS_FEATURES_OK;
    assert_eq!(st.status_writes, vec![0, ack, drv, fok, fok | STATUS_DRIVER_OK]);
    assert_eq!(st.driver_features, VIRTIO_F_VERSION_1);
    let q = st.queues[2];
    assert_eq!(
        (q.enable, q.size, q.desc, q.driver, q.device),
        (1, 16, h.va, h.va + 0x400, h.va + 0x800)
    );
    assert!(q.enabled_at_driver_ok);
    assert_eq!(
        (st.queues[0].enable, st.queues[1].enable),
        (0, 0),
        "control and event queues left off"
    );
    assert_eq!((l.size, l.doorbell, l.max_target, l.max_lun), (16, 8, 255, 16383));
    assert_eq!(l.max_bytes, 65536, "0xFFFF sectors is more than the buffer");
    let r = m.rest.borrow();
    assert_eq!((r.cfg.cdb_size, r.cfg.sense_size), (32, 96));
}

#[test]
fn request_sizes_left_otherwise_are_set_back_to_the_layout() {
    let m = device(vec![]);
    {
        let mut r = m.rest.borrow_mut();
        r.cfg.cdb_size = 16;
        r.cfg.sense_size = 32;
    }
    hba(&m);
    let r = m.rest.borrow();
    assert_eq!((r.cfg.cdb_size, r.cfg.sense_size), (32, 96));
}

#[test]
fn a_device_that_takes_fewer_sectors_bounds_the_buffer() {
    let m = device(vec![]);
    m.rest.borrow_mut().cfg.max_sectors = 16;
    assert_eq!(hba(&m).1.max_bytes, 8192);
}

#[test]
fn refusals_mark_the_device_failed() {
    let m = device(vec![]);
    m.rest.borrow_mut().cfg.num_queues = 0;
    let (c, d, _) = regs(&m);
    assert_eq!(handshake::run(&c, &d, AREA, grant()), Err("no request queue"));
    assert!(m.common.state().status & STATUS_FAILED != 0);

    let m = device(vec![]);
    {
        let mut r = m.rest.borrow_mut();
        r.cfg.sizes_fixed = true;
        r.cfg.cdb_size = 16;
    }
    let (c, d, _) = regs(&m);
    assert_eq!(handshake::run(&c, &d, AREA, grant()), Err("request sizes not taken"));
    assert!(m.common.state().status & STATUS_FAILED != 0);

    let m = device(vec![]);
    m.common.with(|s| s.queues[2].max = 0);
    let (c, d, _) = regs(&m);
    assert!(handshake::run(&c, &d, AREA, grant()).is_err(), "no queue 2");
    let m = device(vec![]);
    m.common.with(|s| s.queues[2].max = 2);
    let (c, d, _) = regs(&m);
    assert!(handshake::run(&c, &d, AREA, grant()).is_err(), "a ring too small for a chain");

    let m = device(vec![]);
    m.common.with(|s| s.offered = 0);
    let (c, d, _) = regs(&m);
    assert!(handshake::run(&c, &d, AREA, grant()).is_err(), "no VERSION_1");
}

#[test]
fn a_ring_smaller_than_asked_is_taken() {
    let m = device(vec![((0, 0), Unit::disk(64, 512))]);
    m.common.with(|s| s.queues[2].max = 4);
    let (_, l) = hba(&m);
    assert_eq!(l.size, 4);
}
