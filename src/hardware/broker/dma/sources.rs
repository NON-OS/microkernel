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

//! Where a DMA map's frames are sought, in order, pure (D6b). A map for a
//! 32-bit device (DMA_MAP_DMA32) looks only below 4 GiB and, with no low
//! memory left, gets none: it is refused, never placed high. alloc.rs
//! takes from each source in turn. kernel_proofs::dma_placement.

/// One place frames come from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Source {
    /// The display pool.
    DisplayPool,
    /// Anywhere, the highest frames first.
    HighFirst,
    /// The reserved low pool, below 4 GiB.
    LowPool,
    /// The general allocator, below 4 GiB.
    Below4G,
    /// The general allocator, anywhere.
    Anywhere,
}

/// The sources a map with these flags tries, first to last. validate.rs
/// refuses a map that asks for both.
pub(crate) const fn sources(high: bool, dma32: bool) -> &'static [Source] {
    use Source::*;
    match (high, dma32) {
        (true, _) => &[DisplayPool, HighFirst],
        (false, true) => &[LowPool, Below4G],
        (false, false) => &[LowPool, Below4G, Anywhere],
    }
}
