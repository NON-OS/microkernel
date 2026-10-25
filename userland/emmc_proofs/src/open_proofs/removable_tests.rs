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

//! A removable slot's open keeps the same order: Bus Master only once a host
//! a dead driver left with ADMA2 running is seen reset, with no DMA region
//! mapped before that, and given back stopped, Bus Master off, the release
//! last. A card put in later is brought up on a host reset again for it.

use nonos_libc::{bus_master_after_reset, bus_master_off_after_stop, released_last, seen};

use super::open_tests::{found, DEAD_TABLE, SLOT};
use crate::emmc::pci::Kind;
use crate::emmc::platform::open_removable;
use crate::model::sd::SdCfg;
use crate::model::{sd_slot, Armed, CardCfg, HostCfg};

#[test]
fn an_empty_removable_slot_left_running_adma_is_reset_before_bus_master() {
    let cfg = HostCfg { cd_inserted: false, ..sd_slot() };
    let card = CardCfg { absent: true, ..CardCfg::default() };
    let host = Armed::left_running(cfg, card, DEAD_TABLE);
    let mut opened = open_removable(&found(Kind::Generic, SLOT)).expect("an empty slot opens");
    bus_master_after_reset(&seen());
    assert!(!opened.slot.describe().present, "no card in the slot");
    let resets = host.borrow().resets;
    host.borrow().sim.0.borrow_mut().put_sd(SdCfg::default());
    assert!(opened.slot.describe().present, "the card put in comes up");
    assert!(host.borrow().resets > resets, "on a host reset for it");
    let violations = host.borrow().sim.0.borrow().violations.clone();
    assert!(violations.is_empty(), "{violations:?}");
    drop(opened);
    let seen = seen();
    bus_master_off_after_stop(&seen);
    released_last(&seen);
}
