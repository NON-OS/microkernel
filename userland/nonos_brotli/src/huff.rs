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

use crate::error::Error;
use alloc::vec::Vec;

/// A fast-table entry for codes longer than eight bits.
pub(crate) const SLOW: u16 = u16::MAX;

/// A canonical prefix code, code bits read most significant first.
pub(crate) struct Code {
    pub(crate) count: [u16; 16],
    pub(crate) syms: Vec<u16>,
    /// Per 8-bit window: symbol << 4 | length, or SLOW for longer codes.
    pub(crate) fast: [u16; 256],
}

impl Code {
    /// The code of a lone symbol, which takes no bits at all.
    pub(crate) fn single(sym: u16) -> Code {
        Code { count: [0; 16], syms: Vec::new(), fast: [sym << 4; 256] }
    }

    /// The code with these lengths per symbol; it must be complete.
    pub(crate) fn build(lens: &[u8]) -> Result<Code, Error> {
        let mut count = [0u16; 16];
        lens.iter().for_each(|&l| count[l as usize] += 1);
        count[0] = 0;
        let kraft: u32 = (1..16).map(|l| (count[l] as u32) << (15 - l)).sum();
        if kraft != 1 << 15 {
            return Err(Error::Invalid);
        }
        let mut syms = Vec::new();
        let used = lens.iter().filter(|&&l| l != 0).count();
        syms.try_reserve_exact(used).map_err(|_| Error::NoMemory)?;
        for l in 1..16u8 {
            (0..lens.len()).filter(|&s| lens[s] == l).for_each(|s| syms.push(s as u16));
        }
        let mut code = Code { count, syms, fast: [SLOW; 256] };
        for v in 0..256u32 {
            if let Some((s, l)) = code.walk(v, 8) {
                code.fast[v as usize] = s << 4 | l as u16;
            }
        }
        Ok(code)
    }
}
