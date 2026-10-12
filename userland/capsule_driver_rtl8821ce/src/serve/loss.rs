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

//! One line on why a link ended, for a laptop with no serial port to show.

use crate::status;

/// One line on why the link ended: the access point's reason code, or none
/// when it went unheard.
pub(super) fn say_loss(left: Option<u16>) {
    match left {
        Some(r) => {
            status::line(b"[rtl8821ce] the access point ended the association, reason ");
            status::number(u32::from(r));
            status::line(b"\n");
        }
        None => status::line(b"[rtl8821ce] the access point went unheard and answered no probe\n"),
    }
}
