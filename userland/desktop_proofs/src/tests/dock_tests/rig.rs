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

//! The dock's apps, windows and measures.

use crate::shell_apps::LAUNCHER_APPS;

pub(super) fn index_of(service: &[u8]) -> usize {
    LAUNCHER_APPS.iter().position(|a| a.service == service).expect("in the table")
}

pub(super) const STORE_PID: u32 = 60;

pub(super) const TERMINAL_PID: u32 = 61;

pub(super) const WINDOW: u32 = 0x5354_4F52;

/// Screen rows of the 800-row screen these tests use: the dock's area from
/// row 713, the bottom edge's band from 796.
pub(super) const HEIGHT: u32 = 800;

pub(super) const DOCK_TOP: u32 = 713;
