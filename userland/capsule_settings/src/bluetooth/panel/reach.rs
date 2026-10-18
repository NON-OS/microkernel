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

//! Whether driver.bt_usb0 could be asked at all.

use super::known::Panel;

/// Whether the service was reached the last time it was asked.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Reach {
    /// The page has not been opened.
    #[default]
    Unasked,
    /// The registry names no driver.bt_usb0: the capsule is not running.
    NoService,
    /// It is named, but did not answer, or answered with no NBTC reply.
    NoAnswer,
    Answered,
}

impl Panel {
    /// The service could not be reached: nothing it said before is shown as
    /// if it still held.
    pub fn lost(&mut self, reach: Reach) {
        let (cursor, notice, id) = (self.cursor, self.notice, self.request_id);
        self.digits.clear();
        *self = Self { reach, cursor, notice, request_id: id, ..Self::new() };
    }
}
