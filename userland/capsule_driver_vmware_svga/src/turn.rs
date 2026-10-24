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

//! What one frame from the inbox is to the serve loop. Only the kernel
//! sends as sender 0, and only to say a device left (DGON): one naming this
//! driver's device ends it, and any other frame from sender 0 is dropped,
//! never answered. Every other sender is served, a DGON's bytes included,
//! so no process can end the driver by sending them.

use super::dgon_parse;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Turn {
    Serve,
    Drop,
    End,
}

pub fn turn(sender: u32, msg: &[u8], device_id: u64) -> Turn {
    match (sender, dgon_parse(msg, sender)) {
        (0, Some(g)) if g.device_id == device_id => Turn::End,
        (0, _) => Turn::Drop,
        _ => Turn::Serve,
    }
}
