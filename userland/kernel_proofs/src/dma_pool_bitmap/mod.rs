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

/*
 * The page bitmaps behind the broker's reserved DMA pools: first-fit runs,
 * the bounds a returned run must sit inside, the count of pages out, and
 * the refusal of a run freed twice, and the pool's size and pressure from
 * the memory map. The arithmetic is included by path.
 */

#[path = "../../../../src/hardware/broker/dma/pool/bitmap.rs"]
pub mod bitmap;
mod free_tests;
#[path = "../../../../src/hardware/broker/dma/pool/run_taken.rs"]
pub mod run_taken;
#[path = "../../../../src/hardware/broker/dma/pool/sizing.rs"]
pub mod sizing;
mod sizing_tests;
mod tests;
