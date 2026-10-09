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
use crate::distance::Ring;
use crate::error::Error;
use crate::lengths::read_command;
use crate::metablock::Meta;
use alloc::vec::Vec;

/// Run the commands of a compressed meta-block until `out` has grown by
/// `len` bytes. `window` is the largest backward distance of the stream.
pub(crate) fn run(
    b: &mut Bits,
    m: &mut Meta,
    out: &mut Vec<u8>,
    (len, window): (usize, usize),
    ring: &mut Ring,
) -> Result<(), Error> {
    let end = out.len() + len;
    loop {
        let t = m.cmd.next(b)?;
        let c = read_command(b, &m.cmd_codes[t])?;
        if c.insert > end - out.len() {
            return Err(Error::Invalid);
        }
        for _ in 0..c.insert {
            crate::literal::literal(b, m, out)?;
        }
        if out.len() == end {
            return Ok(());
        }
        let (dist, code) = match c.same_distance {
            true => (ring.last(), 0),
            false => {
                let t = m.dist.next(b)?;
                let tree = m.dist_map[4 * t + c.copy.min(5) - 2] as usize;
                let code = m.dist_codes[tree].read(b)? as usize;
                (ring.distance(b, code, m.dparams)?, code)
            }
        };
        let limit = window.min(out.len());
        if dist as usize > limit {
            crate::word::emit(out, c.copy, (dist as usize, limit), end)?;
        } else {
            if c.copy > end - out.len() {
                return Err(Error::Invalid);
            }
            if code != 0 {
                ring.push(dist);
            }
            let from = out.len() - dist as usize;
            for i in from..from + c.copy {
                out.push(out[i]);
            }
        }
        if out.len() == end {
            return Ok(());
        }
    }
}
