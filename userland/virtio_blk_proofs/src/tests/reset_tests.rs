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

//! Stopping the device, on either transport: zero written to the status
//! and read back by the clock, the legacy ring's page number cleared after
//! it; a device that never stops is reported, not waited on for good.

use nonos_devmodel::FakeBar;
use nonos_libc::now;
use nonos_virtio::common::DEVICE_STATUS;

use super::bench::{bench, predecessor};
use super::model::view;
use crate::constants::{LEG_QUEUE_NOTIFY, LEG_QUEUE_PFN, LEG_STATUS, RESET_MS};
use crate::regs::Regs;
use crate::transport::{Modern, Transport};

#[test]
fn a_legacy_reset_zeroes_the_status_and_the_ring_page() {
    let dev = bench(8, 2048, 1);
    predecessor(&dev, 3);
    assert!(Transport::Legacy(Regs::pio(5)).reset());
    let d = dev.borrow();
    assert_eq!((d.status, d.pfn, d.seen_stopped, d.written_mid_reset), (0, 0, true, false));
}

#[test]
fn a_device_that_never_stops_is_reported_by_the_clock() {
    let dev = bench(8, 2048, 1);
    predecessor(&dev, u32::MAX);
    assert!(!Transport::Legacy(Regs::pio(5)).reset());
    assert!(now() >= RESET_MS, "gave up after {} ms", now());
}

#[test]
fn a_modern_reset_writes_zero_and_a_device_asking_for_one_is_named() {
    let (common, notify, isr) = (FakeBar::new(0x40), FakeBar::new(0x40), FakeBar::new(4));
    let m = Modern { common: view(&common), notify: view(&notify), doorbell: 4, isr: view(&isr) };
    common.present8(DEVICE_STATUS, 0x4F);
    assert!(Transport::Modern(m).needs_reset(), "DEVICE_NEEDS_RESET went unread");
    assert!(Transport::Modern(m).reset());
    assert_eq!(common.wrote8(DEVICE_STATUS), 0);
    assert!(!Transport::Modern(m).needs_reset());
    notify.present16(4, 0xFFFF);
    Transport::Modern(m).notify(0);
    assert_eq!(notify.wrote16(4), 0, "the request queue's own doorbell");
}

#[test]
fn the_legacy_doorbell_and_status_are_where_the_layout_puts_them() {
    let bar = FakeBar::new(64);
    let legacy = Transport::Legacy(Regs::mmio(view(&bar)));
    bar.present16(LEG_QUEUE_NOTIFY, 0xFFFF);
    legacy.notify(0);
    assert_eq!(bar.wrote16(LEG_QUEUE_NOTIFY), 0);
    bar.present8(LEG_STATUS, 0x40);
    bar.present32(LEG_QUEUE_PFN, 0x1234);
    assert!(legacy.needs_reset());
    assert!(legacy.reset());
    assert_eq!((bar.wrote8(LEG_STATUS), bar.wrote32(LEG_QUEUE_PFN)), (0, 0));
}
