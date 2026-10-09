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

pub(super) const UARTDR: u64 = 0x000;
pub(super) const UARTFR: u64 = 0x018;
pub(super) const UARTIBRD: u64 = 0x024;
pub(super) const UARTFBRD: u64 = 0x028;
pub(super) const UARTLCR_H: u64 = 0x02C;
pub(super) const UARTCR: u64 = 0x030;
pub(super) const UARTIMSC: u64 = 0x038;
pub(super) const UARTMIS: u64 = 0x040;
pub(super) const UARTICR: u64 = 0x044;

pub(super) const FR_BUSY: u32 = 1 << 3;
pub(super) const FR_RXFE: u32 = 1 << 4;
pub(super) const FR_TXFF: u32 = 1 << 5;

pub(super) const CR_UARTEN: u32 = 1 << 0;
pub(super) const CR_TXE: u32 = 1 << 8;
pub(super) const CR_RXE: u32 = 1 << 9;

pub(super) const LCR_FEN: u32 = 1 << 4;
pub(super) const LCR_WLEN_8: u32 = 0b11 << 5;

pub(super) const IMSC_RXIM: u32 = 1 << 4;
pub(super) const INTERRUPT_CLEAR_ALL: u32 = 0x7FF;
