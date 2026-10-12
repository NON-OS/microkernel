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

//! The primary plane's stride register per layout.

use super::gen::PlaneKind;

impl PlaneKind {
    /// Bytes per line from the stride register: DSPSTRIDE holds bytes in
    /// 15:6; PLANE_STRIDE 64 byte units in 9:0, or 10:0 from Gen11.
    pub fn stride_bytes(self, reg: u32) -> u32 {
        match self {
            PlaneKind::Primary => reg & 0xFFC0,
            PlaneKind::Universal => (reg & 0x3FF) * 64,
            PlaneKind::Universal11 => (reg & 0x7FF) * 64,
        }
    }

    /// The stride register's value for `bytes`, a multiple of 64.
    pub fn stride_reg(self, bytes: u32) -> u32 {
        match self {
            PlaneKind::Primary => bytes,
            _ => bytes / 64,
        }
    }

    pub fn max_stride(self) -> u32 {
        self.stride_bytes(u32::MAX)
    }
}
