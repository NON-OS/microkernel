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

//! A card swapped for another between two requests.

use super::removable::empty;
use crate::emmc::error::EmmcError;
use crate::model::sd::{sdsc, SdCfg};

#[test]
fn a_card_swapped_between_requests_is_never_written_as_the_old_one() {
    let (rig, mut slot) = empty();
    rig.sim.borrow_mut().put_sd(SdCfg::default());
    slot.describe();
    {
        let mut sim = rig.sim.borrow_mut();
        sim.take_card();
        sim.put_sd(sdsc());
    }
    let block = vec![0x5a; 512];
    let r = slot.data_disk().map(|d| d.write(0, 1, &block));
    assert_eq!(r.err(), Some(EmmcError::NoMedium), "the latched removal stops it");
    assert_eq!(slot.describe().sectors, ((0xf1f + 1) << 9 << 10) / 512);
    let names = slot.card_disk().expect("the new card").names();
    assert_eq!(names.model(), b"SD SD02G");
}

#[test]
fn a_card_swapped_during_a_request_is_noticed_on_the_next() {
    let (rig, mut slot) = empty();
    rig.sim.borrow_mut().put_sd(SdCfg::default());
    slot.describe();
    let at = rig.cmds().len() + 1;
    rig.sim.borrow_mut().cfg.swap_at_cmd = Some(at);
    let block = vec![0x5a; 512];
    let _ = slot.data_disk().map(|d| d.write(0, 1, &block));
    // The removal latched while a command ran; issuing the next command
    // must not clear it, or the identical card is taken for the old one.
    let r = slot.data_disk().map(|d| d.write(0, 1, &block));
    assert_eq!(r.err(), Some(EmmcError::NoMedium), "the swap went unnoticed");
}
