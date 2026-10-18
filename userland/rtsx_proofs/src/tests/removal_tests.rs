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

//! A card pulled during a transfer ends the transfer at once: the slot is
//! watched on every poll (Linux's rtsx_pci_isr finishes it with
//! TRANS_NO_DEVICE), the engines stop, and no CMD12 goes to an empty slot.

use nonos_libc::now_ms;

use super::rig::{knobs, sdhc, up};
use crate::card::{bring_up, read_blocks};
use crate::model::{with, Knobs};

#[test]
fn a_card_pulled_mid_read_ends_the_read_at_once() {
    let mut drv = up(Some(sdhc(0x1234_5678)));
    let card = bring_up(&mut drv).expect("the card comes up");
    knobs(Knobs { pull_on_data: true, ..Knobs::default() });
    with(|r| r.sent.clear());
    let t0 = now_ms();
    assert!(read_blocks(&drv, &card, 0, 8).is_err());
    assert!(now_ms() - t0 < 50, "the read took {} ms", now_ms() - t0);
    with(|r| assert!(!r.sent.contains(&12), "CMD12 to an empty slot: {:?}", r.sent));
    with(|r| assert!(r.cmd_stopped && r.dma_stopped, "engines left armed"));
}
