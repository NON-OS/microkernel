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

//! The relaunch rig's app and window.

use crate::shell_apps::LAUNCHER_APPS;

pub(super) const WINDOW: u32 = 0x4E4F_0001;

pub(super) fn index_of(service: &[u8]) -> usize {
    LAUNCHER_APPS.iter().position(|a| a.service == service).expect("app in the table")
}
