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
use crate::blocks::Blocks;
use crate::cmap::{read_map, zeroed};
use crate::error::Error;
use crate::header::read_count;
use crate::huff::Code;
use crate::prefix::read_codes;
use alloc::vec::Vec;

/// The header of a compressed meta-block (RFC 7932 9.2): block types,
/// distance parameters, context modes and maps, and the prefix codes.
pub(crate) struct Meta {
    pub(crate) lit: Blocks,
    pub(crate) cmd: Blocks,
    pub(crate) dist: Blocks,
    /// NPOSTFIX and NDIRECT.
    pub(crate) dparams: (u32, usize),
    /// The context mode of each literal block type.
    pub(crate) modes: Vec<u8>,
    pub(crate) lit_map: Vec<u8>,
    pub(crate) dist_map: Vec<u8>,
    pub(crate) lit_codes: Vec<Code>,
    pub(crate) cmd_codes: Vec<Code>,
    pub(crate) dist_codes: Vec<Code>,
}

pub(crate) fn read_meta(b: &mut Bits) -> Result<Meta, Error> {
    let (lit, cmd, dist) = (Blocks::read(b)?, Blocks::read(b)?, Blocks::read(b)?);
    let postfix = b.read(2)?;
    let direct = (b.read(4)? << postfix) as usize;
    let mut modes = zeroed(lit.types)?;
    for m in modes.iter_mut() {
        *m = b.read(2)? as u8;
    }
    let lit_trees = read_count(b)?;
    let lit_map = read_map(b, lit_trees, 64 * lit.types)?;
    let dist_trees = read_count(b)?;
    let dist_map = read_map(b, dist_trees, 4 * dist.types)?;
    let lit_codes = read_codes(b, lit_trees, 256)?;
    let cmd_codes = read_codes(b, cmd.types, 704)?;
    let dist_codes = read_codes(b, dist_trees, 16 + direct + (48 << postfix))?;
    Ok(Meta {
        lit,
        cmd,
        dist,
        dparams: (postfix, direct),
        modes,
        lit_map,
        dist_map,
        lit_codes,
        cmd_codes,
        dist_codes,
    })
}
