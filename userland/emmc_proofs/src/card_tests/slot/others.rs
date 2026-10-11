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

//! A card that will not come up, the write-protect switch, and a fixed
//! slot, which never looks at card detect.

use super::removable::empty;
use crate::emmc::slot::Slot;
use crate::model::sd::SdCfg;
use crate::model::{sd_slot, CardCfg, HostCfg, Rig};

#[test]
fn a_card_that_will_not_come_up_costs_one_attempt_until_it_leaves() {
    let (rig, mut slot) = empty();
    rig.sim.borrow_mut().put_sd(SdCfg { ready_after: u32::MAX, ..SdCfg::default() });
    assert!(!slot.describe().present);
    let tries = || rig.sim.borrow().sd.as_ref().map_or(0, |c| c.acmd41_args.len());
    let once = tries();
    for _ in 0..3 {
        assert!(!slot.describe().present);
    }
    assert_eq!(tries(), once, "not tried again while it stays");
    rig.sim.borrow_mut().take_card();
    slot.describe();
    rig.sim.borrow_mut().put_sd(SdCfg::default());
    assert!(slot.describe().present, "a new card is tried");
}

#[test]
fn the_write_protect_switch_makes_the_card_read_only() {
    let rig = Rig::empty_slot(HostCfg { write_protected: true, ..sd_slot() });
    let mut slot = Slot::removable(rig.parts(false), rig.data).expect("host up");
    rig.sim.borrow_mut().put_sd(SdCfg::default());
    let m = slot.describe();
    assert!(m.present && m.read_only);
    assert!(!slot.writable());
}

#[test]
fn a_fixed_slot_is_always_present_and_never_polled() {
    let rig = Rig::new(HostCfg { write_protected: true, ..HostCfg::default() }, CardCfg::default());
    let mut slot = Slot::fixed(rig.parts(true), rig.data).expect("bring-up");
    let before = rig.cmds().len();
    let m = slot.describe();
    assert!(m.present && !m.removable && !m.read_only, "no switch is wired to an eMMC");
    assert!(slot.data_disk().is_ok());
    assert_eq!(rig.cmds().len(), before);
}
