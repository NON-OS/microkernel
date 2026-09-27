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

//! One large allocation, every page touched, the shape of a STARK prover's
//! buffers: hundreds of megabytes in one piece, not a heap of small ones.

const MIB: usize = 1 << 20;
/// Large enough to need the big-allocation path, small enough for the 2 GiB
/// test machine beside the desktop.
pub const SIZE: usize = 256 * MIB;
const PAGE: usize = 4096;

pub fn prove() -> Result<String, String> {
    let mut buf: Vec<u8> = Vec::new();
    buf.try_reserve_exact(SIZE).map_err(|e| format!("reserve {} MiB: {e}", SIZE / MIB))?;
    buf.resize(SIZE, 0);
    for (i, page) in buf.chunks_mut(PAGE).enumerate() {
        page[0] = (i % 251) as u8;
    }
    let bad = buf.chunks(PAGE).enumerate().find(|(i, p)| p[0] != (*i % 251) as u8);
    match bad {
        Some((i, _)) => Err(format!("page {i} lost its byte")),
        None => Ok(format!("{} MiB in one allocation, {} pages touched", SIZE / MIB, SIZE / PAGE)),
    }
}
