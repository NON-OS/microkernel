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

//! The session protection notification the responder sends.

use crate::gen3_model::Out;

/// The session protection notification the firmware sends.
pub fn session_notif(status: u32, start: u32) -> Out {
    let p: Vec<u8> = [0u32, status, start, 0].iter().flat_map(|w| w.to_le_bytes()).collect();
    Out { cmd: 0xFB, group: 0x3, seq: 0x8000, payload: p }
}

/// How the responder answers session protection.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SessionReply {
    Start,
    Refuse,
    Never,
}
