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

//! Only the Settings app may change the machine's DHCP lease
//! (`capsule_net_dhcp/src/server/lease_admin.rs`).

use crate::lease_admin::{may_change_lease, LEASE_ADMINS};

const SETTINGS: u32 = 40;

fn registry(name: &[u8]) -> Option<u32> {
    match name {
        b"app.settings" => Some(SETTINGS),
        b"terminal" => Some(41),
        _ => None,
    }
}

#[test]
fn settings_is_the_one_admin() {
    assert_eq!(LEASE_ADMINS, &[b"app.settings" as &[u8]]);
    assert!(may_change_lease(SETTINGS, registry));
}

#[test]
fn any_other_client_is_refused() {
    for pid in [1, 41, SETTINGS + 1, u32::MAX] {
        assert!(!may_change_lease(pid, registry), "pid {pid}");
    }
}

#[test]
fn with_settings_not_running_nobody_changes_the_lease() {
    for pid in [1, SETTINGS, u32::MAX] {
        assert!(!may_change_lease(pid, |_| None), "pid {pid}");
    }
}

#[test]
fn pid_zero_never_changes_the_lease() {
    assert!(!may_change_lease(0, |_| Some(0)));
}
