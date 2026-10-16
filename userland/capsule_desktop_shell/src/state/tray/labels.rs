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

//! The tray's labels as the menu bar reads them.

use super::table::TrayTable;
use crate::protocol::TRAY_LABEL_MAX;

impl TrayTable {
    /// Every live item's label, in table order: what the menu bar shows.
    pub fn labels(&self) -> impl Iterator<Item = &[u8]> {
        self.entries
            .iter()
            .filter(|e| e.in_use)
            .map(|e| &e.label[..(e.label_len as usize).min(TRAY_LABEL_MAX)])
    }
}
