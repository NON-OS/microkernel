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

//! A launch the installer refused ends with the installer's own status, so
//! the toast says why (launch_say.rs); no outcome kept says no more than a
//! load that finished without the app. The probe asks the installer's op 9.

use super::settle::fresh;
use crate::launch::Step;
use crate::state::Uptime;

const POLL: &str =
    include_str!("../../../../capsule_desktop_shell/src/server/handlers/installed_launch_poll.rs");
const PROBE: &str = include_str!("../../../../capsule_desktop_shell/src/installer_client/probe.rs");
const OPS: &str =
    include_str!("../../../../capsule_desktop_shell/src/installer_client/constants.rs");

#[test]
fn a_refused_launch_ends_with_the_installers_status() {
    for status in [-13, -22, -1] {
        let mut l = fresh();
        l.probed(Uptime(1_500), Some(status));
        assert_eq!(l.step(Uptime(1_500), None), Step::Refused(status), "status {status}");
    }
    let mut l = fresh();
    l.probed(Uptime(1_500), Some(-2));
    assert_eq!(l.step(Uptime(1_500), None), Step::Failed, "no outcome kept");
}

#[test]
fn the_poll_asks_after_its_own_launch_and_says_the_refusal() {
    let poll: String = POLL.split_whitespace().collect();
    assert!(poll.contains("launch.probed(now,probe(&launch.name));"));
    assert!(poll.contains("Step::Refused(status)=>{ifletSome(launch)=ctx.store_apps.launch"));
    assert!(poll.contains("report_failure(ctx,&launch.name,status);"));
    assert!(PROBE.contains("request(OP_LOAD_OUTCOME, name)"));
    assert!(OPS.contains("pub(super) const OP_LOAD_OUTCOME: u16 = 9;"));
}
