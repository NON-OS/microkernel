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

//! How each question to the device is retried, and the UIC commands the
//! link stage builds.

use crate::model::{Cfg, Rig};
use crate::ufs::error::UfsError;
use crate::ufs::facts::uic::*;
use crate::ufs::host::uic::UicCmd;

/// A refused query is asked again, three times in all; a silent NOP OUT
/// is not (device_tests).
#[test]
fn a_refused_query_is_asked_again_up_to_three_times() {
    let rig = Rig::new(Cfg::default());
    rig.m.dev.borrow_mut().refuse_queries = 2;
    assert!(rig.up(false).is_ok());
    let rig = Rig::new(Cfg::default());
    rig.m.dev.borrow_mut().refuse_queries = 3;
    assert_eq!(rig.up(false).err(), Some(UfsError::Query(0xFE)));
    assert_eq!(rig.m.dev.borrow().queries, 3, "the fDeviceInit set, three times");
    assert!(rig.m.said("fDeviceInit failed: query response"));
}

#[test]
fn uic_commands_carry_their_attribute_in_the_upper_half() {
    assert_eq!(UicCmd::link_startup(), UicCmd { op: DME_LINKSTARTUP, args: [0; 3] });
    let get = UicCmd::get(PA_LOCAL_TX_LCC_ENABLE);
    assert_eq!((get.op, get.args), (DME_GET, [0x155E_0000, 0, 0]));
    let set = UicCmd::set(PA_LOCAL_TX_LCC_ENABLE, 0);
    assert_eq!((set.op, set.args), (DME_SET, [0x155E_0000, 0, 0]));
}
