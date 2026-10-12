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

//! Mapping a contiguous run of IOVA pages into the GGTT without a list of
//! them: the entries are written in chunks from a stack array, each chunk
//! through `ggtt::map` with its checks and its flush.

use super::access::Mmio;
use super::error::Error;
use super::ggtt;
use super::ggtt_pte::Pte;

const CHUNK: u32 = 64;

/// Entries `first..first + pages` to the pages from `base` on.
pub fn map_run(
    m: &mut dyn Mmio,
    t: Pte,
    first: u32,
    base: u64,
    pages: u32,
    total: u32,
) -> Result<(), Error> {
    let mut done = 0;
    while done < pages {
        let n = CHUNK.min(pages - done);
        let mut run = [0u64; CHUNK as usize];
        for (i, e) in run[..n as usize].iter_mut().enumerate() {
            *e = base + (done as u64 + i as u64) * 4096;
        }
        ggtt::map(m, t, first + done, &run[..n as usize], total)?;
        done += n;
    }
    Ok(())
}
