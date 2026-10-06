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

use super::plane::Plane;
use crate::image::jpeg::bits::BitReader;
use crate::image::types::DecodeError;

/// A correction bit for a coefficient that is already nonzero.
pub(super) fn correct(
    br: &mut BitReader,
    p: &mut Plane,
    blk: usize,
    k: usize,
    p1: i32,
) -> Result<(), DecodeError> {
    let c = p.get(blk, k) as i32;
    if br.read_bits(1)? == 1 && c & p1 == 0 && c != 0 {
        p.set(blk, k, super::blocks::clamp16(if c >= 0 { c + p1 } else { c - p1 }));
    }
    Ok(())
}
