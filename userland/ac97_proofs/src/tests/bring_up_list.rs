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

//! The descriptor list the bring-up writes, and an engine firmware left
//! running.

use nonos_libc::dma_at;

use super::util::{up, up_with};
use crate::constants::ring::{BDL_ENTRIES, BD_BUP, PERIOD_BYTES, PERIOD_SAMPLES};
use crate::model::{Options, NABM_GRANT};

#[test]
fn the_list_walks_the_four_periods_eight_times() {
    let (d, _ich) = up();
    let list = dma_at(d.bdl_dev() as u64) as *const u32;
    for i in 0..BDL_ENTRIES {
        // SAFETY: the list is inside the grant `dma_at` found.
        let (addr, ctl) = unsafe { (*list.add(2 * i), *list.add(2 * i + 1)) };
        assert_eq!(addr, d.ring_dev() + ((i % 4) * PERIOD_BYTES) as u32, "entry {i}");
        assert_eq!(ctl, BD_BUP | PERIOD_SAMPLES, "entry {i}");
    }
}

#[test]
fn an_engine_firmware_left_running_is_stopped_before_the_link_is_reset() {
    let (d, ich) = up_with(Options { running: true, ..Options::default() });
    assert!(d.is_ok());
    let ich = ich.borrow();
    assert_eq!((ich.cr & 1, ich.sr & 1), (0, 1), "stopped and halted");
    let at = |off: u16| ich.writes.iter().position(|w| w.0 == NABM_GRANT && w.1 == off);
    assert!(at(0x1B).unwrap() < at(0x2C).unwrap(), "stopped before the link is touched");
}
