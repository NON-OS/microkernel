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

//! The descriptor list and the sample ring, in one coherent DMA grant: the
//! list in the first page, the ring after it. Thirty-two descriptors cycle
//! over four periods; a period is 2048 frames of 16-bit stereo at 48 kHz,
//! about 43 ms, the same period the HD Audio driver plays.

pub const BDL_ENTRIES: usize = 32;
pub const BDL_OFFSET: u64 = 0;
pub const RING_OFFSET: u64 = 0x1000;

pub const N_PERIODS: usize = 4;
pub const PERIOD_BYTES: usize = 0x2000;
pub const RING_BYTES: usize = N_PERIODS * PERIOD_BYTES;
pub const GRANT_BYTES: u64 = RING_OFFSET + RING_BYTES as u64;

/// A descriptor counts sixteen-bit samples, both channels.
pub const PERIOD_SAMPLES: u32 = (PERIOD_BYTES / 2) as u32;
/// Buffer underrun policy: a halted engine sends zeros rather than its last
/// sample over and over. No completion interrupt is asked for.
pub const BD_BUP: u32 = 1 << 30;
/// The index the last-valid register trails the current one by: every entry
/// but the one just played stays valid, so the engine never halts while the
/// driver polls.
pub const LVI_LAG: u8 = (BDL_ENTRIES - 1) as u8;
pub const INDEX_MASK: u8 = (BDL_ENTRIES - 1) as u8;
