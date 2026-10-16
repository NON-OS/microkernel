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

//! Bus mastering on the modern bring-up (plan/50 C2), over a register file
//! per queue (`banked`) that a reset leaves on a dead predecessor's rings:
//! Bus Master goes on only once the device is reset and handshaken and both
//! queues are on this driver's grants (`master_modern_refused_tests`).

use std::cell::Cell;

use nonos_virtio::common::{accept, start_until, NotifyArea, DEVICE_FEATURE, DEVICE_STATUS};
use nonos_virtio::common::{QUEUE_DESC, QUEUE_DEVICE, QUEUE_DRIVER, QUEUE_ENABLE};

use super::banked::Banked;
use super::bus_master::noted;
use crate::clock::expired_after;
use crate::constants::{MODERN_WANTED, Q_RX, Q_TX, RESET_MS};
use crate::modern::{program::program, spec::spec};

pub(super) const RX: u64 = 0x20_0000;
pub(super) const TX: u64 = RX + 0x3000;
pub(super) const AREA: NotifyArea = NotifyArea { multiplier: 4, len: 0x100 };
/// ACKNOWLEDGE, DRIVER and FEATURES_OK; and with DRIVER_OK, a device live.
const SETTLED: u8 = 0x0B;
const LIVE: u8 = 0x0F;

/// A device a predecessor left live on its own rings, offering VERSION_1
/// and queues of `max`, handshaken by the steps that come before them.
pub(super) fn left_live_and_handshaken(max: [u16; 2]) -> Banked {
    let c = Banked::sized(max);
    c.common.put(DEVICE_FEATURE, &1u32.to_le_bytes());
    c.common.put(DEVICE_STATUS, &[LIVE]);
    for q in &c.queues {
        q.put(QUEUE_ENABLE, &1u16.to_le_bytes());
        q.put(QUEUE_DESC, &0xBAD_000u32.to_le_bytes());
    }
    start_until(&c, expired_after(RESET_MS)).expect("reset read back");
    accept(&c, MODERN_WANTED).expect("features");
    c
}

/// Queue `q` as the device holds it: enabled, and its three parts' places.
fn ring(c: &Banked, q: usize) -> [u32; 4] {
    let at = |off| c.queues[q].u32_at(off);
    [at(QUEUE_ENABLE) & 0xFFFF, at(QUEUE_DESC), at(QUEUE_DRIVER), at(QUEUE_DEVICE)]
}

/// What it should hold for queue `index` on this driver's ring at `base`.
fn ours(index: u16, base: u64) -> [u32; 4] {
    let s = spec(index, base).expect("a ring in range");
    [1, s.desc as u32, s.driver as u32, s.device as u32]
}

#[test]
fn a_device_left_live_masters_the_bus_only_handshaken_and_on_this_drivers_rings() {
    let c = left_live_and_handshaken([256; 2]);
    let at_master = Cell::new(None);
    let master = || noted(&at_master, (c.common.get(DEVICE_STATUS), [ring(&c, 0), ring(&c, 1)]));
    assert!(program(&c, AREA, (RX, TX), master).is_ok());
    let both = [ours(Q_RX, RX), ours(Q_TX, TX)];
    assert_eq!(at_master.get(), Some((SETTLED, both)), "Bus Master on stale rings");
}
