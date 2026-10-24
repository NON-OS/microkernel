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

//! The host: HCE that never comes back, the lists never ready, and DMA the
//! host cannot reach, each stopping the bring-up by name before the device
//! is spoken to.

use crate::model::{Cfg, Rig};
use crate::ufs::error::UfsError;
use crate::ufs::facts::bounds::HCE_ON_MS;
use crate::ufs::host::hce::HCE_STUCK_OFF;

#[test]
fn a_host_that_never_enables_is_named_after_its_bound() {
    let rig = Rig::new(Cfg { hce_never: true, ..Cfg::default() });
    assert_eq!(rig.up(false).err(), Some(UfsError::Host));
    assert!(rig.m.said(HCE_STUCK_OFF));
    assert!(rig.m.said("host enable failed: host not ready"));
    assert!(rig.m.now.get() >= HCE_ON_MS, "waited its bound");
    assert!(rig.m.host.borrow().uic.is_empty(), "no link without a host");
}

#[test]
fn an_initialising_host_is_waited_for() {
    let rig = Rig::new(Cfg { hce_settle: 30, ..Cfg::default() });
    assert!(rig.up(false).is_ok());
}

#[test]
fn lists_that_never_come_ready_are_not_run() {
    let rig = Rig::new(Cfg { lists_never: true, ..Cfg::default() });
    assert_eq!(rig.up(false).err(), Some(UfsError::Host));
    assert!(rig.m.said("transfer lists failed: host not ready"));
    let h = rig.m.host.borrow();
    assert_eq!(h.regs[(crate::ufs::facts::hci::UTRLRSR / 4) as usize], 0);
    assert!(h.utrds.is_empty());
}

/// A host without 64-bit addressing and a region above 4 GiB: refused
/// before the link, rather than DMA landing at the address cut to 32 bits.
#[test]
fn dma_the_host_cannot_reach_stops_it_before_the_link() {
    let rig = Rig::new(Cfg { cap: 31, ..Cfg::default() });
    let mut l = rig.lists();
    l.data.bus = 1 << 33;
    let r = crate::ufs::bring_up::bring_up(&rig.m, l, false);
    assert_eq!(r.err(), Some(UfsError::Dma32));
    assert!(rig.m.host.borrow().uic.is_empty());
    assert!(rig.m.said("DMA failed: DMA above 4 GiB and no 64-bit addressing"));
}
