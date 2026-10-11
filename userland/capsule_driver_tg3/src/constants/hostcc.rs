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

//! Host coalescing: when the part writes the status block back (tg3.h), and
//! tg3_init_coal's values for a 5705_PLUS part.

pub const HOSTCC_MODE: usize = 0x3C00;
pub const HOSTCC_MODE_ENABLE: u32 = 0x0000_0002;
pub const HOSTCC_MODE_NOW: u32 = 0x0000_0008;
/// tp->coalesce_mode for every part but the 5700 AX/BX.
pub const HOSTCC_MODE_32BYTE: u32 = 0x0000_0100;
pub const HOSTCC_RXCOL_TICKS: usize = 0x3C08;
pub const HOSTCC_TXCOL_TICKS: usize = 0x3C0C;
pub const HOSTCC_RXMAX_FRAMES: usize = 0x3C10;
pub const HOSTCC_TXMAX_FRAMES: usize = 0x3C14;
pub const HOSTCC_RXCOAL_MAXF_INT: usize = 0x3C20;
pub const HOSTCC_TXCOAL_MAXF_INT: usize = 0x3C24;
pub const HOSTCC_STATUS_BLK_HOST_ADDR: usize = 0x3C38;
/// The first vector's block; vector n is `n * VEC_STRIDE` past it.
pub const HOSTCC_RXCOL_TICKS_VEC1: usize = 0x3D80;
pub const HOSTCC_TXCOL_TICKS_VEC1: usize = 0x3D84;
pub const HOSTCC_RXMAX_FRAMES_VEC1: usize = 0x3D88;
pub const HOSTCC_TXMAX_FRAMES_VEC1: usize = 0x3D8C;
pub const HOSTCC_RXCOAL_MAXF_INT_VEC1: usize = 0x3D90;
pub const HOSTCC_TXCOAL_MAXF_INT_VEC1: usize = 0x3D94;
pub const VEC_STRIDE: usize = 0x18;
/// TG3_IRQ_MAX_VECS, the vectors a 57765_PLUS part has (SUPPORT_MSIX).
pub const IRQ_MAX_VECS: usize = 5;

pub const LOW_RXCOL_TICKS: u32 = 0x32;
pub const LOW_TXCOL_TICKS: u32 = 0x96;
pub const LOW_RXMAX_FRAMES: u32 = 0x05;
pub const LOW_TXMAX_FRAMES: u32 = 0x35;
pub const DEFAULT_RXCOAL_MAXF_INT: u32 = 0x05;
pub const DEFAULT_TXCOAL_MAXF_INT: u32 = 0x05;
