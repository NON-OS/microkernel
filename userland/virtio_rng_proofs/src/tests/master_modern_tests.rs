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

//! Bus mastering on the modern bring-up, over a common
//! configuration a reset leaves on a dead predecessor's ring. Bus Master
//! (`master`) goes on only once the device is reset, handshaken and on this
//! driver's grant, DRIVER_OK after it; never on a refusal at its queue.

use std::cell::Cell;

use nonos_devmodel::FakeBar;
use nonos_virtio::common::{accept, start_until, NotifyArea, DEVICE_FEATURE, DEVICE_STATUS};
use nonos_virtio::common::{QUEUE_DESC, QUEUE_ENABLE, QUEUE_NOTIFY_OFF, QUEUE_SIZE};

use super::bus_master::noted;
use super::model::view;
use crate::clock::expired_after;
use crate::constants::RESET_MS;
use crate::modern_queue::start;

const RING: u64 = 0x30_0000;
const AREA: NotifyArea = NotifyArea { multiplier: 4, len: 0x40 };
/// ACKNOWLEDGE, DRIVER and FEATURES_OK; with DRIVER_OK, a device live.
const SETTLED: u8 = 0x0B;
const LIVE: u8 = 0x0F;

/// A device a predecessor left live, offering VERSION_1 and a queue of
/// `max`, reset and handshaken as bring-up runs it.
fn left_live_and_handshaken(max: u16, notify_off: u16) -> FakeBar {
    let c = FakeBar::new(0x40);
    c.present32(DEVICE_FEATURE, 1);
    c.present16(QUEUE_SIZE, max);
    c.present16(QUEUE_NOTIFY_OFF, notify_off);
    c.present8(DEVICE_STATUS, LIVE);
    c.present16(QUEUE_ENABLE, 1);
    c.present32(QUEUE_DESC, 0xBAD_000);
    start_until(&view(&c), expired_after(RESET_MS)).expect("reset read back");
    accept(&view(&c), 0).expect("features");
    c
}

#[test]
fn a_device_left_live_masters_the_bus_only_handshaken_and_on_this_drivers_ring() {
    let c = left_live_and_handshaken(64, 0);
    let at_master = Cell::new(None);
    let master = || noted(&at_master, (c.wrote8(DEVICE_STATUS), c.wrote32(QUEUE_DESC)));
    assert!(start(&view(&c), AREA, RING, master).is_ok());
    assert_eq!(at_master.get(), Some((SETTLED, RING as u32)), "Bus Master on a stale ring");
    assert_eq!(c.wrote8(DEVICE_STATUS), LIVE, "DRIVER_OK after Bus Master");
}

#[test]
fn a_device_refused_at_its_queue_never_masters_the_bus() {
    // No queue, a doorbell past the notify region, a ring whose parts wrap.
    for (max, notify_off, ring) in [(0, 0, RING), (64, 0x100, RING), (64, 0, u64::MAX - 0xFF)] {
        let c = left_live_and_handshaken(max, notify_off);
        let mastered = Cell::new(None);
        assert!(start(&view(&c), AREA, ring, || noted(&mastered, ())).is_err(), "max {max}");
        assert!(mastered.get().is_none(), "Bus Master on a refusal path: max {max} ring {ring:#x}");
        assert_eq!(c.wrote8(DEVICE_STATUS) & (LIVE ^ SETTLED), 0, "DRIVER_OK on a refused device");
    }
}
