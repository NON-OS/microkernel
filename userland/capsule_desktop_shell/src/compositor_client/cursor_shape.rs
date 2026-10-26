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
//! The pointer's shape over the shell's chrome: the arrow while the pointer
//! is over the shell's own pixels, its say withdrawn over the transparent
//! rest, where a window below shows through and has its own say.

use super::wire::call;

const OP: u16 = 0x0018; // OP_CURSOR_SHAPE
const BODY_LEN: usize = 16;
/// The compositor's arrow (frame_pacer/cursor_shapes.rs ARROW_CODE).
const ARROW: u32 = 0;
/// The compositor's withdraw (requests/pointer.rs CURSOR_WITHDRAW).
const WITHDRAW: u32 = 0xFF;

/// Ask for the arrow over `handle` (`over`), or withdraw the ask.
pub fn cursor_shape(
    port: u32,
    request_id: u32,
    handle: u64,
    over: bool,
) -> Result<(), &'static str> {
    let mut body = [0u8; BODY_LEN];
    body[0..8].copy_from_slice(&handle.to_le_bytes());
    let code = if over { ARROW } else { WITHDRAW };
    body[8..12].copy_from_slice(&code.to_le_bytes());
    match call(port, OP, request_id, &body)? {
        0 => Ok(()),
        _ => Err("compositor rejected cursor_shape"),
    }
}
