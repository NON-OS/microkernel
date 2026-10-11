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

//! A removable slot served with and without a card, through the real slot
//! code against the model host and card.

use crate::emmc::error::EmmcError;
use crate::emmc::slot::Slot;
use crate::model::sd::SdCfg;
use crate::model::{sd_slot, Rig, SimBus, TestClock, TestLog};

pub type TestSlot = Slot<SimBus, TestClock, TestLog>;

pub fn empty() -> (Rig, TestSlot) {
    let rig = Rig::empty_slot(sd_slot());
    let slot = Slot::removable(rig.parts(false), rig.data).expect("host up");
    (rig, slot)
}

#[test]
fn an_empty_slot_is_served_and_asked_nothing() {
    let (rig, mut slot) = empty();
    let m = slot.describe();
    assert!(m.removable && !m.present && !m.read_only && m.sectors == 0);
    assert_eq!(slot.data_disk().err(), Some(EmmcError::NoMedium));
    assert_eq!(slot.card_disk().err(), Some(EmmcError::NoMedium));
    assert!(rig.cmds().is_empty(), "no command to an empty slot");
    assert!(!rig.logged("forced present"), "card detect is the truth");
}

#[test]
fn a_card_put_in_is_brought_up_and_data_waits_for_geometry() {
    let (rig, mut slot) = empty();
    rig.sim.borrow_mut().put_sd(SdCfg::default());
    assert_eq!(slot.data_disk().err(), Some(EmmcError::NoMedium), "not yet described");
    let m = slot.describe();
    assert!(m.present && m.removable && m.sectors == (0xe4df + 1) * 1024);
    let mut buf = vec![0u8; 512];
    slot.data_disk().expect("described").read(5, 1, &mut buf).expect("read");
    assert!(slot.card_disk().expect("a card").card.sd);
    rig.assert_clean();
}

#[test]
fn a_card_taken_out_is_dropped() {
    let (rig, mut slot) = empty();
    rig.sim.borrow_mut().put_sd(SdCfg::default());
    slot.describe();
    rig.sim.borrow_mut().take_card();
    assert_eq!(slot.data_disk().err(), Some(EmmcError::NoMedium));
    assert!(!slot.describe().present);
    assert!(rig.logged("card removed"));
}
