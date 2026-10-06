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

use crate::bits::Bits;
use crate::error::Error;
use crate::prefix::read_code;
use alloc::vec::Vec;

/// A zeroed table of `size` bytes, or `NoMemory`.
pub(crate) fn zeroed(size: usize) -> Result<Vec<u8>, Error> {
    let mut v = Vec::new();
    v.try_reserve_exact(size).map_err(|_| Error::NoMemory)?;
    v.resize(size, 0);
    Ok(v)
}

/// A context map of `size` tree indices below `trees` (RFC 7932 7.3):
/// runs of zeros coded by length, then an optional move-to-front.
pub(crate) fn read_map(b: &mut Bits, trees: usize, size: usize) -> Result<Vec<u8>, Error> {
    let mut map = zeroed(size)?;
    if trees < 2 {
        return Ok(map);
    }
    let rle = match b.read(1)? {
        1 => b.read(4)? as usize + 1,
        _ => 0,
    };
    let code = read_code(b, trees + rle)?;
    let mut i = 0;
    while i < size {
        match code.read(b)? as usize {
            0 => i += 1,
            s if s <= rle => i += (1 << s) + b.read(s as u32)? as usize,
            s => {
                map[i] = (s - rle) as u8;
                i += 1;
            }
        }
    }
    if i > size {
        return Err(Error::Invalid);
    }
    if b.read(1)? == 1 {
        let mut order: [u8; 256] = core::array::from_fn(|k| k as u8);
        for v in map.iter_mut() {
            let (k, x) = (*v as usize, order[*v as usize]);
            order.copy_within(0..k, 1);
            (order[0], *v) = (x, x);
        }
    }
    Ok(map)
}
