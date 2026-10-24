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
//! A bulk pair's memory, for a slot to keep whole when a failed controller
//! may have been given it: both rings, every stream ring and its context
//! array, and both pages.
use alloc::vec::Vec;

use super::pipes::BulkPipes;
use super::streams::StreamPipe;
use crate::dma::DmaRegion;

impl StreamPipe {
    pub fn into_regions(self) -> impl Iterator<Item = DmaRegion> {
        let StreamPipe { array, rings, .. } = self;
        core::iter::once(array).chain(rings.into_iter().flat_map(|r| r.into_regions()))
    }
}

impl BulkPipes {
    pub fn into_regions(self) -> impl Iterator<Item = DmaRegion> {
        let mut all: Vec<DmaRegion> = Vec::new();
        all.extend(self.in_ring.into_regions());
        all.extend(self.out_ring.into_regions());
        all.extend([self.buf, self.poll_buf]);
        all.extend(self.in_streams.into_iter().flat_map(StreamPipe::into_regions));
        all.extend(self.out_streams.into_iter().flat_map(StreamPipe::into_regions));
        all.into_iter()
    }
}
