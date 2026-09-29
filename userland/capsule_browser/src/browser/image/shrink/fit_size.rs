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

use super::super::store::Decoded;
use super::Shrink;

/// `d` box-filtered to `size`, or as it is when that is its size.
pub(in super::super) fn fit(d: Decoded, size: (u32, u32)) -> Result<Decoded, &'static str> {
    let size = (size.0.clamp(1, d.w.max(1)), size.1.clamp(1, d.h.max(1)));
    if (d.w, d.h) == size {
        return Ok(d);
    }
    let mut s = Shrink::new((d.w, d.h), size).ok_or("no memory for the raster")?;
    for (y, row) in d.px.chunks(d.w.max(1) as usize).enumerate() {
        s.row(y, row);
    }
    Ok(s.finish())
}
