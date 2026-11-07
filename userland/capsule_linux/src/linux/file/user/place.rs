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

//! Which guest paths are the person's folders.
//!
//! The guest's home is /root (env.rs). Its Documents, Downloads, Music,
//! Pictures and Videos are the broker's, under the same names, for a
//! program whose policy grants FILES_USER; for any other they are what
//! they were before, the family's home or the shared tree. Paths reach here made absolute, with `.` and
//! `..` resolved and clamped at `/` (resolve::visible), so the broker is
//! never asked to resolve a climb.

use nonos_linux_policy::mask::FILES_USER;

use crate::linux::guest::Guest;

pub use super::place_rule::{broker_path, FOLDERS, HOME};

/// The broker's path for `visible` when this guest is served the folders.
pub fn served<'a>(guest: &Guest, visible: &'a [u8]) -> Option<&'a [u8]> {
    if !crate::linux::app::granted(guest, FILES_USER) {
        return None;
    }
    broker_path(visible)
}

/// Whether `visible` is the home the folders are listed in, for a guest
/// served them.
pub fn is_home(guest: &Guest, visible: &[u8]) -> bool {
    visible == HOME && crate::linux::app::granted(guest, FILES_USER)
}
