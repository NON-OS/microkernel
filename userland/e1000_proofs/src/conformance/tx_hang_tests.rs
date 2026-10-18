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

//! The transmit watchdog against a part whose transmitter stops: a stall past
//! the bound with the link up resets the part and starts both rings over;
//! with the link down, or a completion seen, nothing is reset.

use nonos_libc::{entropy, logged, set_command, Deadline, MK_PCI_CMD_MEMORY_SPACE};

use super::live::{live_part, Live};
use super::memory::{Memory, RX_RING_PHYS};
use super::model::window;
use crate::constants::regs::{REG_RCTL, REG_RDBAL, REG_RDT, REG_STATUS, REG_TCTL};
use crate::constants::status::{RCTL_EN, STATUS_LU, TCTL_EN};
use crate::server::tx_watch::check;
use crate::setup::Driver;

/// Three frames posted that the part never completes, the deadline past.
pub fn stalled(link: u32) -> (std::sync::Arc<nonos_devmodel::FakeBar>, Live, Driver, Memory) {
    let bar = window();
    let part = live_part(&bar);
    set_command(MK_PCI_CMD_MEMORY_SPACE);
    let mut mem = Memory::new();
    let mut d = crate::init::finish(mem.driver(&bar)).expect("brought up");
    bar.present32(REG_STATUS, link);
    for _ in 0..3 {
        d.tx.post(60);
    }
    d.tx_watch = Deadline::after_ms(0);
    (bar, part, d, mem)
}

#[test]
fn a_transmitter_stalled_with_the_link_up_is_reset_and_its_rings_restarted() {
    let _turn = entropy(true);
    let (bar, part, mut d, _mem) = stalled(STATUS_LU);
    let resets = part.resets();
    assert!(check(&mut d, false), "a stall past the bound is a hang");
    assert_eq!(part.resets(), resets + 1, "the part was reset");
    assert_eq!((d.tx.tail, d.tx.clean, d.rx.head), (0, 0, 0), "rings start over");
    assert_eq!(bar.wrote32(REG_RDBAL), RX_RING_PHYS as u32);
    assert_eq!(bar.wrote32(REG_RDT), 31, "every receive slot handed back");
    assert_ne!(bar.wrote32(REG_RCTL) & RCTL_EN, 0);
    assert_ne!(bar.wrote32(REG_TCTL) & TCTL_EN, 0);
    let line = "driver.e1000: tx hang, reset: done\n";
    assert_eq!(logged().last().map(String::as_str), Some(line));
    assert!(!check(&mut d, false), "an empty ring is no hang");
}

#[test]
fn a_full_ring_with_the_link_down_or_a_completion_seen_is_left_alone() {
    let _turn = entropy(true);
    let (_bar, part, mut d, _mem) = stalled(0);
    let resets = part.resets();
    assert!(!check(&mut d, false), "the part sends once the link returns");
    assert!(!d.tx_watch.expired(), "the clock starts again from link down");
    d.tx_watch = Deadline::after_ms(0);
    assert!(!check(&mut d, true), "a completion is a sign of life");
    assert!(!d.tx_watch.expired());
    assert_eq!(part.resets(), resets, "nothing was reset");
    assert_eq!(d.tx.tail, 3, "the frames stay queued");
}
