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

//! The control and status registers the driver touches, per the 21140A and
//! 21143 hardware reference manuals. CSR `n` sits at `n * 8`.

pub const CSR0: u16 = 0;
pub const CSR1: u16 = 1;
pub const CSR2: u16 = 2;
pub const CSR3: u16 = 3;
pub const CSR4: u16 = 4;
pub const CSR5: u16 = 5;
pub const CSR6: u16 = 6;
pub const CSR7: u16 = 7;

/// Bus mode: software reset, which the part clears when done.
pub const CSR0_SWR: u32 = 1 << 0;
/// Bus mode after the reset: little-endian, descriptors back to back (DSL
/// 0), bursts of up to eight longwords.
pub const CSR0_BUS_MODE: u32 = 8 << 8;

/// Status: the receive and transmit process states, and a fatal bus error.
pub const CSR5_RS_SHIFT: u32 = 17;
pub const CSR5_TS_SHIFT: u32 = 20;
pub const CSR5_STATE_MASK: u32 = 7;
pub const CSR5_FBE: u32 = 1 << 13;
/// Every status bit a one clears.
pub const CSR5_CLEAR_ALL: u32 = 0x0C01_FFFF;

/// Operation mode: receive and transmit started. Nothing else: no
/// promiscuous or pass-all-multicast mode, perfect filtering as the setup
/// frame loads it.
pub const CSR6_SR: u32 = 1 << 1;
pub const CSR6_ST: u32 = 1 << 13;
