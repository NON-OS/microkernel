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

//! K.1, K.2: a Linux run's served endpoint is held to one capsule
//! (SERVE_ONE), so only the owner of that capsule's endpoint reaches it, by
//! name or port, by pid, or by the run role's own name. Before held_to, any
//! capsule holding IPC did.

use super::common::{owner, Gate, IPC, NETWORK};
use super::held::{endpoint_admits, inbox_admits};
use super::registry_addressed::addressed_admits;

const SERVED: &str = "linux.serve.app.simplex";
const HOLD: Option<&str> = Some("app.simplex");
/// The run role's process once it serves: its role, its reply inbox, and
/// the served endpoint, all read from its one inbox.
const RUN: [Gate; 3] = [
    ("app.linux.run.net", IPC, None),
    ("endpoint.app.linux.run.net.reply", IPC, None),
    (SERVED, IPC, HOLD),
];

#[test]
fn a_held_endpoint_admits_only_its_capsules_owner() {
    assert!(endpoint_admits(SERVED, HOLD, owner(&["app.simplex"])), "the window");
    for mine in [
        &[][..],
        &["app.attack", "endpoint.app.attack.reply"],
        &["app.linux.run.net"],
        &["app.simplex.1", "simplex"],
        &["desktop_shell", "compositor", "net.core"],
    ] {
        assert!(!endpoint_admits(SERVED, HOLD, owner(mine)), "{mine:?} reached the core");
    }
    assert!(endpoint_admits(SERVED, None, owner(&[])), "unheld, it is as it was");
    // The list comes first: a hold never opens a held driver.
    assert!(!endpoint_admits("driver.e1000_0", Some("app.attack"), owner(&["app.attack"])));
}

#[test]
fn a_send_to_the_holder_by_pid_or_its_role_takes_the_hold() {
    assert!(!inbox_admits(RUN, IPC, owner(&[])), "IPC alone wrote into the run");
    let other = owner(&["app.attack"]);
    assert!(!inbox_admits(RUN, IPC | NETWORK, other), "another capsule wrote into it");
    assert!(inbox_admits(RUN, IPC, owner(&["app.simplex"])), "the window by pid");
    // By the role's own name: another capsule's endpoint, so its inbox's gates.
    let by_role = |mine: &'static [&'static str]| {
        addressed_admits(false, false, || inbox_admits(RUN, IPC, owner(mine)))
    };
    assert!(!by_role(&["app.attack"]), "app.linux.run.net let another capsule in");
    assert!(by_role(&["app.simplex"]));
    // Before it serves, the run is reached as it was.
    assert!(inbox_admits(RUN[..2].iter().copied(), IPC, owner(&[])));
}
