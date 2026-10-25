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

//! Bytes over a link a slice at a time: each read or write asks the proxy
//! once, for a slice.

use super::super::link::Link;
use super::SLICE_MS;

/// What one slice read: how many bytes, and whether the far end has said
/// it finished.
pub struct Read {
    pub n: usize,
    pub ended: bool,
}

impl Link {
    /// Carry the front of `data`; how many bytes went. Zero means offer the
    /// same bytes again on a later slice.
    pub fn write_slice(&mut self, data: &[u8]) -> Result<usize, ()> {
        self.stream.write_slice(data, SLICE_MS).map_err(|_| ())
    }

    pub fn read_slice(&mut self, into: &mut [u8]) -> Result<Read, ()> {
        let n = self.stream.read_slice(into, SLICE_MS).map_err(|_| ())?;
        Ok(Read { n, ended: n == 0 && self.stream.ended() })
    }
}
