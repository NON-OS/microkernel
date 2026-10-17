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
//! Reading a command's body, and the one body the host answers with.

use super::gpu::Res;
use crate::constants::{VG_MAX_SCANOUTS, VG_RESP_OK_DISPLAY_INFO};
use crate::tests::part::answers::header;

/// The panel the host reports on scanout 0.
pub const PANEL: (u32, u32) = (640, 480);

pub fn w(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(b[24 + at..28 + at].try_into().unwrap())
}

pub fn fits(r: &Res, b: &[u8]) -> bool {
    let (x, y, rw, rh) = (w(b, 0) as u64, w(b, 4) as u64, w(b, 8) as u64, w(b, 12) as u64);
    rw > 0 && rh > 0 && x + rw <= r.w as u64 && y + rh <= r.h as u64
}

/// Scanout 0 enabled at PANEL, the rest off.
pub fn display_info() -> Vec<u8> {
    let mut out = header(VG_RESP_OK_DISPLAY_INFO, 0, 0);
    for v in [0, 0, PANEL.0, PANEL.1, 1, 0] {
        out.extend_from_slice(&v.to_le_bytes());
    }
    out.resize(24 + 24 * VG_MAX_SCANOUTS, 0);
    out
}
