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

use crate::wire::{call_status, NWMP_MAGIC};

const OP: u16 = 0x000E;
const BODY_LEN: usize = 24;
/// The window manager's MAXIMIZE_FLAG_FULL_SCREEN (window/full_screen.rs).
const FLAG_FULL_SCREEN: u32 = 1;

/// Tell the window manager the window took `x, y, w, h`. `full_screen` says
/// it is the green button's full screen, which hides the dock while the
/// window shows; false is the restore of the saved rect.
pub fn window_maximize(
    port: u32,
    request_id: u32,
    window_id: u32,
    (x, y, w, h): (u32, u32, u32, u32),
    full_screen: bool,
) -> Result<(), &'static str> {
    let mut body = [0u8; BODY_LEN];
    body[0..4].copy_from_slice(&window_id.to_le_bytes());
    let flags = if full_screen { FLAG_FULL_SCREEN } else { 0 };
    body[4..8].copy_from_slice(&flags.to_le_bytes());
    body[8..12].copy_from_slice(&x.to_le_bytes());
    body[12..16].copy_from_slice(&y.to_le_bytes());
    body[16..20].copy_from_slice(&w.to_le_bytes());
    body[20..24].copy_from_slice(&h.to_le_bytes());
    let status = call_status(port, NWMP_MAGIC, OP, request_id, &body)?;
    if status != 0 {
        return Err("wm rejected window_maximize");
    }
    Ok(())
}
