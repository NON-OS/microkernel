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

//! A modern queue refused at bring-up (plan/50 C2): the receive queue may
//! already be enabled over its ring, in a grant the failed attempt releases
//! next, so the device is reset before the error goes back. A bring-up
//! whose queues are both taken is not reset.

use nonos_virtio::common::{NotifyArea, DEVICE_STATUS, QUEUE_ENABLE, QUEUE_SIZE};

use super::mem::Mem;
use crate::modern::program::program;

const RX_AT: u64 = 0x20_0000;
const TX_AT: u64 = RX_AT + 0x3000;
/// ACKNOWLEDGE, DRIVER and FEATURES_OK: where bring-up stands at the queues.
const LIVE: u8 = 0x0B;
const AREA: NotifyArea = NotifyArea { multiplier: 4, len: 0x100 };

/// A common configuration whose queues offer `max` entries.
fn common(max: u16) -> Mem {
    let c = Mem::new(0x40);
    c.put(QUEUE_SIZE, &max.to_le_bytes());
    c.put(DEVICE_STATUS, &[LIVE]);
    c
}

#[test]
fn a_refused_transmit_queue_resets_the_device_after_the_receive_queue_was_enabled() {
    let c = common(256);
    assert!(program(&c.view(), AREA, RX_AT, TX_AT + 8).is_err(), "an unaligned ring");
    assert_eq!(c.u16_at(QUEUE_ENABLE), 1, "the receive queue was enabled first");
    assert_eq!(c.get(DEVICE_STATUS), 0, "not reset before the release");
}

#[test]
fn every_refusal_resets_the_device() {
    let cases = [(256, u64::MAX - 0x1000), (256, !0xFFF), (64, TX_AT), (0, TX_AT), (255, TX_AT)];
    for (max, tx_at) in cases {
        let c = common(max);
        assert!(program(&c.view(), AREA, RX_AT, tx_at).is_err(), "max {max} tx {tx_at:#x}");
        assert_eq!(c.get(DEVICE_STATUS), 0, "max {max} tx {tx_at:#x}: not reset");
    }
}

#[test]
fn queues_both_taken_leave_the_device_as_it_was() {
    let c = common(1024);
    let sizes = program(&c.view(), AREA, RX_AT, TX_AT).map(|[rx, tx]| (rx.size, tx.size));
    assert_eq!(sizes, Ok((256, 256)), "told the 256 entries the layout holds");
    assert_eq!(c.get(DEVICE_STATUS), LIVE);
}
