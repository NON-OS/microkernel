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

//! The disconnect reply, and a reply of a status code alone.

use super::super::connect::{disconnect, Session};
use super::super::radio::Radio;

pub(super) fn disconnect_reply(
    radio: &mut Radio,
    session: &mut Option<Session>,
    out: &mut [u8],
) -> Option<usize> {
    let status = match radio {
        Radio::Up(up) => {
            up.link.note_left();
            disconnect(&mut up.link, &mut up.keys, &up.regs, session)
        }
        Radio::Down => -1,
    };
    code_reply(status, out)
}

pub(super) fn code_reply(status: i32, out: &mut [u8]) -> Option<usize> {
    out[10..14].copy_from_slice(&status.to_le_bytes());
    Some(14)
}
