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

//! NCMP 0x0013 LID: closed darkens every output, open wakes them.

use super::wire::call;

const OP: u16 = 0x0013;

/// Tell the compositor the screen is to be `dark`; Ok once it answered.
pub fn lid(compositor_port: u32, request_id: u32, dark: bool) -> Result<i32, &'static str> {
    call(compositor_port, OP, request_id, &u32::from(dark).to_le_bytes())
}
