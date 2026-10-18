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

//! Bus mastering on the modern bring-up, for a device refused at its
//! queues (plan/50 C2): it is left reset and never masters the bus, from
//! the same predecessor's state as `master_modern_tests`.

use std::cell::Cell;

use nonos_virtio::common::DEVICE_STATUS;

use super::bus_master::noted;
use super::master_modern_tests::{left_live_and_handshaken, AREA, RX, TX};
use crate::modern::program::program;

#[test]
fn a_device_refused_at_its_queues_never_masters_the_bus() {
    // No receive queue, no transmit queue, one under 256 on either, a ring
    // not page aligned, a ring that wraps.
    let sizes = [[0, 256], [256, 0], [128, 256], [256, 128]].map(|max| (max, TX));
    for (max, tx) in sizes.into_iter().chain([([256; 2], TX + 8), ([256; 2], u64::MAX - 0x1000)]) {
        let c = left_live_and_handshaken(max);
        let mastered = Cell::new(None);
        assert!(program(&c, AREA, (RX, tx), || noted(&mastered, ())).is_err(), "{max:?}");
        assert!(mastered.get().is_none(), "Bus Master on a refusal path: {max:?} tx {tx:#x}");
        assert_eq!(c.common.get(DEVICE_STATUS), 0, "{max:?} tx {tx:#x}: not left reset");
    }
}
