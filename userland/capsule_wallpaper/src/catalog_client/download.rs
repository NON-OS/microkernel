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

//! One wallpaper coming over from the catalog, a chunk per call, kept between
//! tries. A try used to be all or nothing: one call past its budget threw
//! away every chunk already fetched, and the next try, a poll later, began
//! again at byte 0 with the size. A wallpaper is 20 to 120 calls, each of
//! which must come back in time on one emulated processor the rest of the
//! desktop is busy on, so a try could keep failing somewhere along the way
//! and the desktop stay on its built in picture. Now what came over stays,
//! and the next try asks for what is still missing. Asking again is safe: the
//! late answer to a call given up on carries that call's token, and the
//! kernel drops it rather than hand it to the next call.

use alloc::vec::Vec;

use super::budget::chunk_ms;
use super::fetch_chunk::fetch_chunk;
use super::fetch_size::fetch_size;
use super::proto::{CHUNK_MAX, HDR_LEN, IPC_PAYLOAD_MAX};

/// The largest wallpaper taken: the biggest in the collection is a quarter
/// of this.
pub const MAX_IMAGE_BYTES: u32 = 2_000_000;

/// Calls one try makes at most: the whole of the largest wallpaper taken.
const MAX_CHUNKS: u32 = (MAX_IMAGE_BYTES / CHUNK_MAX as u32) + 2;

pub struct Download {
    index: u8,
    size: u32,
    bytes: Vec<u8>,
}

impl Download {
    /// Begin wallpaper `index` with the catalog's size for it, or `None` when
    /// it gives none, or one the service does not take.
    pub fn start(catalog_port: u32, index: u8) -> Option<Download> {
        let size = fetch_size(catalog_port, index as u32)?;
        if size == 0 || size > MAX_IMAGE_BYTES {
            return None;
        }
        let mut bytes = Vec::new();
        bytes.try_reserve_exact(size as usize).ok()?;
        Some(Download { index, size, bytes })
    }

    /// The wallpaper this is.
    pub fn index(&self) -> u8 {
        self.index
    }

    /// Bytes fetched so far, and the whole.
    pub fn progress(&self) -> (u32, u32) {
        (self.bytes.len() as u32, self.size)
    }

    /// One try: fetch what is still missing. True once the wallpaper is
    /// whole; false when a call failed, keeping every byte before it.
    pub fn resume(&mut self, catalog_port: u32) -> bool {
        let mut buf = [0u8; IPC_PAYLOAD_MAX];
        for call in 0..MAX_CHUNKS {
            let offset = self.bytes.len() as u32;
            if offset >= self.size {
                break;
            }
            let budget = chunk_ms(call == 0);
            let Some(len) = fetch_chunk(catalog_port, self.index as u32, offset, &mut buf, budget)
            else {
                return false;
            };
            if offset.checked_add(len).is_none_or(|end| end > self.size) {
                return false;
            }
            self.bytes.extend_from_slice(&buf[HDR_LEN..HDR_LEN + len as usize]);
        }
        self.bytes.len() as u32 == self.size
    }

    /// The whole wallpaper, once `resume` has said it is.
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}
