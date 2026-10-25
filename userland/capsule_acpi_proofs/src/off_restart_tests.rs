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

//! The press rule after a restart (F-107).

use crate::off_rule::*;

#[path = "shutdown_say_tests.rs"]
mod shutdown_say;

/// F-107: a restarted capsule has no boot guard (the kernel served the
/// button while it was away), but a bounce is still a bounce.
#[test]
fn a_restart_has_no_boot_guard() {
    for v in [Press::PowerOff, Press::Bounce, Press::BootGuard] {
        assert_eq!(on_restart(v, false), v, "a first start keeps every verdict");
        let want = if v == Press::Bounce { Press::Bounce } else { Press::PowerOff };
        assert_eq!(on_restart(v, true), want);
    }
    assert_eq!(on_restart(press(None, None, 10), true), Press::PowerOff, "before any pass");
    assert_eq!(on_restart(press(Some(9_800), None, 10_000), true), Press::Bounce);
}
