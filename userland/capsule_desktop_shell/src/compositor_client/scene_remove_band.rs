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

//! SCENE_REMOVE with BAND_ONLY: the shell's layer in one band dropped, its
//! others kept (the chrome moving in or out of the trusted band).

use super::wire::call_boot;

const OP: u16 = 0x0007;
const BAND_ONLY: u32 = 1;

pub fn remove_band(compositor_port: u32, request_id: u32, z: u32) -> Result<(), &'static str> {
    let mut body = [0u8; 8];
    body[..4].copy_from_slice(&z.to_le_bytes());
    body[4..].copy_from_slice(&BAND_ONLY.to_le_bytes());
    match call_boot(compositor_port, OP, request_id, &body)? {
        0 => Ok(()),
        _ => Err("compositor refused the band remove"),
    }
}
