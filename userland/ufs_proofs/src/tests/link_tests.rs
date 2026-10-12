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

//! Link startup: retried three times with the host reset between, a device
//! required on the link, and Intel's hosts with LCC cleared first.

use crate::model::{Cfg, Rig};
use crate::ufs::error::UfsError;
use crate::ufs::facts::uic::*;

#[test]
fn two_failed_startups_are_retried_after_a_host_reset() {
    let rig = Rig::new(Cfg { link_fails: 2, ..Cfg::default() });
    assert!(rig.up(false).is_ok());
    let h = rig.m.host.borrow();
    assert_eq!((h.link_tries, h.enables), (3, 3), "a reset before each retry");
}

#[test]
fn four_failures_are_the_end_with_the_uic_result() {
    let rig = Rig::new(Cfg { link_fails: 4, ..Cfg::default() });
    assert_eq!(rig.up(false).err(), Some(UfsError::Uic(1)));
    assert_eq!(rig.m.host.borrow().link_tries, 4, "DME_LINKSTARTUP_RETRIES is 3");
    assert!(rig.m.said("link startup failed: UIC result 0x1"));
}

#[test]
fn a_link_with_no_device_is_said_so() {
    let rig = Rig::new(Cfg { no_device: true, ..Cfg::default() });
    assert_eq!(rig.up(false).err(), Some(UfsError::NoDevice));
    assert!(rig.m.said("link startup failed: no device"));
}

#[test]
fn intel_hosts_have_lcc_cleared_only_when_set() {
    let rig = Rig::new(Cfg { lcc: 1, link_fails: 1, ..Cfg::default() });
    assert!(rig.up(true).is_ok());
    let h = rig.m.host.borrow();
    let mib = (PA_LOCAL_TX_LCC_ENABLE as u32) << 16;
    let sets: Vec<_> = h.uic.iter().filter(|(c, a)| *c == DME_SET && a[0] == mib).collect();
    assert_eq!(sets.len(), 1, "set once, then found clear on the retry");
    assert_eq!(sets[0].1[2], 0);
    assert_eq!(h.lcc, 0);
    drop(h);
    let clear = Rig::new(Cfg::default());
    assert!(clear.up(true).is_ok());
    assert!(clear.m.host.borrow().uic.iter().all(|(c, _)| *c != DME_SET));
    let other = Rig::new(Cfg { lcc: 1, ..Cfg::default() });
    assert!(other.up(false).is_ok());
    assert!(other.m.host.borrow().uic.iter().all(|(c, _)| *c == DME_LINKSTARTUP));
}
