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

//! What Ctrl+C copies: a link the core just made, else our address, else the
//! open chat's newest message. A person shares a link by pasting it.

use super::says::LINK_NOTICE;
use super::state::Model;

impl Model {
    pub fn copy_text(&self) -> Option<&str> {
        if let Some(link) = self.notice.strip_prefix(LINK_NOTICE) {
            return Some(link);
        }
        if let Some(address) = &self.address {
            return Some(address);
        }
        self.current()?.last().map(|i| i.text.as_str())
    }
}
