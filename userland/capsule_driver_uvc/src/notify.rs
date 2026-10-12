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
//! The compositor told whenever the camera starts or stops streaming, so
//! the indicator shows it (NCMP version 1, op 000Ah, body `kind u32` (1,
//! the camera) and `on u32`). The answer does not change the stream: a
//! compositor that does not know the op yet answers -22.

use nonos_libc::{mk_ipc_call, mk_service_lookup};

const MAGIC: u32 = 0x4E43_4D50; // "NCMP"
const VERSION: u16 = 1;
const OP_CAPTURE: u16 = 0x000A;
const CAMERA: u32 = 1;
const COMPOSITOR: &str = "compositor";

pub fn capture(on: bool) {
    let (mut port, mut pid) = (0u32, 0u32);
    if mk_service_lookup(COMPOSITOR.as_ptr(), COMPOSITOR.len(), &mut port, &mut pid) != 0 {
        return;
    }
    let mut req = [0u8; 28];
    req[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    req[4..6].copy_from_slice(&VERSION.to_le_bytes());
    req[6..8].copy_from_slice(&OP_CAPTURE.to_le_bytes());
    req[16..20].copy_from_slice(&8u32.to_le_bytes());
    req[20..24].copy_from_slice(&CAMERA.to_le_bytes());
    req[24..28].copy_from_slice(&(on as u32).to_le_bytes());
    let mut resp = [0u8; 32];
    let _ = mk_ipc_call(port as u64, req.as_ptr(), req.len(), resp.as_mut_ptr(), resp.len());
}
