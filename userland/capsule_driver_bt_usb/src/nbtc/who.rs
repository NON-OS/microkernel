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

//! Who may ask what (the ruling on E-25). The kernel holds driver.bt_usb0
//! to Settings and the shell; this capsule checks each op again, by the
//! sender's pid against whoever holds each name now, and denies by default.
//! Settings sees everything but keys; the shell sees only the link's state
//! and battery levels, and never an identity address.

use super::ops::{OP_BATTERY, OP_HEALTH, OP_LINK};

/// Settings, in its first window or a later one
/// (`src/userspace/capsule_settings/spawn.rs`).
pub const SETTINGS: [&[u8]; 3] = [b"app.settings", b"app.settings.1", b"app.settings.2"];
pub const SHELL: &[u8] = b"desktop_shell";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Caller {
    Settings,
    Shell,
    Other,
}

/// Who `sender` is, given `owner_of`, the pid that holds a name. Pid 0 is
/// the kernel, which holds no name here.
pub fn caller(sender: u32, owner_of: impl Fn(&[u8]) -> Option<u32>) -> Caller {
    let holds = |name: &[u8]| sender != 0 && owner_of(name) == Some(sender);
    if SETTINGS.iter().any(|name| holds(name)) {
        Caller::Settings
    } else if holds(SHELL) {
        Caller::Shell
    } else {
        Caller::Other
    }
}

/// Whether `who` may ask `op`. An op this table does not know is Settings'
/// to be refused as unknown; nobody else learns that it is unknown.
pub fn may(who: Caller, op: u16) -> bool {
    match who {
        Caller::Settings => true,
        Caller::Shell => matches!(op, OP_HEALTH | OP_LINK | OP_BATTERY),
        Caller::Other => false,
    }
}
