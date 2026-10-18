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
//! The SSP-type SPI core of an Intel LPSS SPI function, its registers as
//! recalled from Intel's PCH and SoC datasheets' LPSS SPI chapter and the
//! PXA SSP lineage it descends from, all unverified. Where recall is weak
//! the comment says so, and the driver only reads what it needs to prove
//! the core alive before it writes.

/// Control 0: frame size, frame format, enable and serial clock rate.
pub const SSCR0: u64 = 0x00;
/// Data size select, bits 3:0: bits per frame less one (4 to 16). The
/// frame format, bits 5:4, is left 0: Motorola SPI.
pub const SSCR0_DSS_MASK: u32 = 0xF;
/// Synchronous serial port enable.
pub const SSCR0_SSE: u32 = 1 << 7;
/// Serial clock rate, bits 19:8: the bit clock is the input clock over
/// SCR + 1.
pub const SSCR0_SCR_SHIFT: u32 = 8;
pub const SSCR0_SCR_MAX: u32 = 0xFFF;

/// Control 1: clock polarity and phase, loopback, FIFO thresholds and the
/// interrupt enables (left clear: this driver polls).
pub const SSCR1: u64 = 0x04;
/// Clock polarity: SCLK idles high.
pub const SSCR1_SPO: u32 = 1 << 3;
/// Clock phase: data sampled on the second edge.
pub const SSCR1_SPH: u32 = 1 << 4;

/// Status: FIFO levels and the busy and overrun flags.
pub const SSSR: u64 = 0x08;
/// Transmit FIFO not full.
pub const SSSR_TNF: u32 = 1 << 2;
/// Receive FIFO not empty.
pub const SSSR_RNE: u32 = 1 << 3;
/// A frame is moving.
pub const SSSR_BSY: u32 = 1 << 4;
/// Receive FIFO overrun, cleared by writing 1.
pub const SSSR_ROR: u32 = 1 << 7;

/// Data: a write queues a frame, a read takes one received.
pub const SSDR: u64 = 0x10;
