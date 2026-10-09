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

use crate::image::png::deflate::{BitReader, ByteSource};
use crate::image::png::huffman::Huffman;
use crate::image::types::DecodeError;

use super::block::block;
use super::dynamic::dynamic;
use super::fixed_litlen::fixed_litlen;
use super::stored::stored;
use super::window::{Out, Sink};

/* Inflate a zlib stream from `src` into `sink`, block by block, holding only
 * the 32 KiB window. Returns once the final block ends or the sink is done;
 * a stream that runs out first is Truncated. */
pub fn inflate_zlib<S: ByteSource, K: Sink>(src: S, sink: &mut K) -> Result<(), DecodeError> {
    let mut bits = BitReader::new(src);
    let cmf = bits.read_bits(8)?;
    let flg = bits.read_bits(8)?;
    if cmf & 0x0f != 8 || flg & 0x20 != 0 || (cmf * 256 + flg) % 31 != 0 {
        return Err(DecodeError::BadMagic);
    }
    let mut out = Out::new(sink);
    loop {
        let final_block = bits.read_bits(1)?;
        match bits.read_bits(2)? {
            0 => stored(&mut bits, &mut out)?,
            1 => {
                let ll = fixed_litlen()?;
                let ds = Huffman::from_lengths(&[5u8; 30])?;
                block(&mut bits, &ll, &ds, &mut out)?;
            }
            2 => {
                let (ll, ds) = dynamic(&mut bits)?;
                block(&mut bits, &ll, &ds, &mut out)?;
            }
            _ => return Err(DecodeError::Unsupported),
        }
        if final_block != 0 || out.done() {
            return Ok(());
        }
    }
}
