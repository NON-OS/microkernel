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

pub type SurfaceSid = u64;
pub type SurfaceHandle = u64;

#[inline]
pub fn encode_handle(slot_idx: u32, epoch: u32) -> SurfaceHandle {
    ((slot_idx as u64) << 32) | epoch as u64
}

#[inline]
pub fn decode_handle(h: SurfaceHandle) -> (u32, u32) {
    ((h >> 32) as u32, h as u32)
}
