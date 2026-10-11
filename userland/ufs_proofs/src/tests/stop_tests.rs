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

//! Stopping the host before its regions go back (the lead's review, R11):
//! HCE written 0 and waited for, so a failed bring-up unmaps nothing the
//! host may still be reading; a host that never stops is reported, and the
//! platform then leaks the regions and releases only the claim.

use crate::model::{Cfg, Rig};
use crate::ufs::hci::enable::stop;
use crate::ufs::hci::regs::HCE;
use crate::ufs::time::HCE_OFF_MS;

#[test]
fn a_host_that_stops_is_seen_stopped() {
    let rig = Rig::new(Cfg::default());
    let _ = rig.up(false);
    assert!(stop(&rig.m));
    assert_eq!(rig.m.host.borrow().regs[(HCE / 4) as usize], 0);
}

#[test]
fn a_host_that_never_stops_is_reported_after_the_whole_wait() {
    let rig = Rig::new(Cfg { hce_stuck_on: true, ..Cfg::default() });
    let _ = rig.up(false);
    let start = rig.m.now.get();
    assert!(!stop(&rig.m));
    assert!(rig.m.now.get() - start >= HCE_OFF_MS);
}
