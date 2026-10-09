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

use super::bits::Bits;
use super::codes::codes;
use super::huff_build::build;
use super::meta;
use super::out::Out;
use super::tables::Codes;
use super::types::End;

/// A block coded with the fixed Huffman codes of RFC 1951 3.2.6.
pub fn fixed(b: &mut Bits, out: &mut Out, c: &mut Codes) -> Result<(), End> {
    let mut ll = [8u8; 288];
    ll[144..256].fill(9);
    ll[256..280].fill(7);
    build(&mut c.lit, &ll, meta::litlen)?;
    build(&mut c.dist, &[5u8; 30], meta::dist)?;
    codes(b, out, &c.lit, &c.dist)
}
