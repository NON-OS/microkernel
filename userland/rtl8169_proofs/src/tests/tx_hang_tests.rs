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

//! The transmit watchdog: the next slot still the part's past the bound with
//! the link up resets the part; link down, or the slot free, resets nothing.

use std::sync::{atomic::AtomicU32, atomic::Ordering::SeqCst, Arc};

use nonos_libc::{entropy, logged, set_command, Deadline};

use super::memory::{Memory, RX_DESC_DA};
use super::model::{live, window, Live};
use super::probe::counting;
use crate::constants::regs::*;
use crate::init::finish;
use crate::queue::desc::{desc, desc_mut, Descriptor};
use crate::server::tx_watch::check;
use crate::setup::Driver;

/// Up, link up, the next slot the part's, the deadline past; and the window.
pub fn stalled(resets: &Arc<AtomicU32>) -> (Arc<nonos_devmodel::FakeBar>, Live, Driver, Memory) {
    let (bar, mut mem) = (window(), Memory::new());
    let part = live(&bar, counting(resets));
    set_command(0);
    let mut d = finish(mem.driver(&bar)).expect("brought up");
    bar.present8(REG_PHY_STATUS, 0x02);
    let owned = Descriptor { opts1: DESC_OWN, opts2: 0, addr_lo: 0, addr_hi: 0 };
    // SAFETY: slot 3 is inside the 16-descriptor ring `mem` holds.
    unsafe { desc_mut(d.tx.desc_va, 3, owned) };
    (d.tx.cur, d.rx.cur, d.tx_watch) = (3, 5, Deadline::after_ms(0));
    (bar, part, d, mem)
}

#[test]
fn a_slot_held_with_the_link_up_past_the_bound_resets_the_part() {
    let (_turn, resets) = (entropy(true), Arc::new(AtomicU32::new(0)));
    let (_bar, _part, mut d, _mem) = stalled(&resets);
    let done = resets.load(SeqCst);
    assert!(check(&mut d, true), "a stall past the bound is a hang");
    assert_eq!(resets.load(SeqCst), done + 1, "the part was reset");
    assert_eq!((d.tx.cur, d.rx.cur), (0, 0), "both rings start over");
    // SAFETY: slot 3 is inside the ring, which the reset left to the driver.
    assert_eq!(unsafe { desc(d.tx.desc_va, 3) }.opts1 & DESC_OWN, 0, "slot rewritten");
    // SAFETY: RxDescAddr and ChipCmd lie inside the window.
    let (rdsar, cmd) = unsafe { (d.regs.r32(REG_RXDESC_ADDR_LO), d.regs.r8(REG_CMD)) };
    assert_eq!((rdsar, cmd & 0x0C), (RX_DESC_DA as u32, 0x0C), "rings named, engines on");
    assert_eq!(logged().last().unwrap(), "rtl8169: tx hang, reset: done\n");
}

#[test]
fn a_slot_held_with_the_link_down_or_freed_is_left_alone() {
    let (_turn, resets) = (entropy(true), Arc::new(AtomicU32::new(0)));
    let (_bar, _part, mut d, _mem) = stalled(&resets);
    let done = resets.load(SeqCst);
    // SAFETY: PHYstatus lies inside the window.
    unsafe { d.regs.w8(REG_PHY_STATUS, 0) };
    assert!(!check(&mut d, true), "the part sends once the link returns");
    assert!(!d.tx_watch.expired(), "the clock starts again from link down");
    d.tx_watch = Deadline::after_ms(0);
    assert!(!check(&mut d, false), "a free slot is a sign of life");
    assert_eq!((resets.load(SeqCst), d.tx.cur), (done, 3), "nothing reset");
}
