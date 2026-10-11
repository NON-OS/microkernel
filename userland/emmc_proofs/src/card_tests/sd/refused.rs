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

//! SD cards the driver does not serve, and the bound on how long it waits.

use crate::emmc::error::EmmcError;
use crate::model::sd::SdCfg;
use crate::model::{sd_slot, Rig};

#[test]
fn a_card_past_two_terabytes_is_refused() {
    let sduc = SdCfg { csd_structure: 2, ..SdCfg::default() };
    let rig = Rig::sd(sd_slot(), sduc);
    let e = rig.disk(false).err().expect("refused");
    assert_eq!(e, EmmcError::NoCapacity);
    assert!(rig.logged("SD csd structure 2 not served"));
}

#[test]
fn a_card_that_never_finishes_power_up_is_given_a_second() {
    let rig = Rig::sd(sd_slot(), SdCfg { ready_after: u32::MAX, ..SdCfg::default() });
    let e = rig.disk(false).err().expect("busy");
    assert_eq!(e, EmmcError::CardBusy);
    assert!(rig.ms() < 3_000, "bounded: {} ms", rig.ms());
}
