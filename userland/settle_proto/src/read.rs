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

//! OP_READ. Request: chain_id u64 | view u16 | n u8 | n words. Reply body:
//! m u8 | m words, the view's raw ABI return.

use alloc::vec::Vec;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Read {
    pub chain_id: u64,
    pub view: u16,
    pub args: Vec<[u8; 32]>,
}

fn words(n: usize, b: &[u8]) -> Option<Vec<[u8; 32]>> {
    if b.len() != 32 * n {
        return None;
    }
    b.chunks_exact(32).map(|w| w.try_into().ok()).collect()
}

impl Read {
    /// None past 255 words.
    pub fn encode(&self) -> Option<Vec<u8>> {
        let n = u8::try_from(self.args.len()).ok()?;
        let mut out = Vec::with_capacity(11 + 32 * self.args.len());
        out.extend_from_slice(&self.chain_id.to_le_bytes());
        out.extend_from_slice(&self.view.to_le_bytes());
        out.push(n);
        self.args.iter().for_each(|w| out.extend_from_slice(w));
        Some(out)
    }

    pub fn parse(b: &[u8]) -> Option<Self> {
        let head = b.get(..11)?;
        Some(Read {
            chain_id: u64::from_le_bytes(head[..8].try_into().ok()?),
            view: u16::from_le_bytes([head[8], head[9]]),
            args: words(head[10] as usize, &b[11..])?,
        })
    }
}

/// READ's reply body; None past 255 words.
pub fn read_reply(ret: &[[u8; 32]]) -> Option<Vec<u8>> {
    let m = u8::try_from(ret.len()).ok()?;
    let mut out = Vec::with_capacity(1 + 32 * ret.len());
    out.push(m);
    ret.iter().for_each(|w| out.extend_from_slice(w));
    Some(out)
}

pub fn parse_read_reply(b: &[u8]) -> Option<Vec<[u8; 32]>> {
    let (m, rest) = b.split_first()?;
    words(*m as usize, rest)
}
