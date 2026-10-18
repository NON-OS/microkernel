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

// Register indices; `Regs` turns them into offsets with the port's shift.
pub(super) const RBR: u64 = 0;
pub(super) const THR: u64 = 0;
pub(super) const DLL: u64 = 0;
pub(super) const DLM: u64 = 1;
pub(super) const IER: u64 = 1;
pub(super) const FCR: u64 = 2;
pub(super) const LCR: u64 = 3;
pub(super) const MCR: u64 = 4;
pub(super) const LSR: u64 = 5;

pub(super) const LSR_DR: u32 = 1 << 0;
pub(super) const LSR_THRE: u32 = 1 << 5;

pub(super) const LCR_DLAB: u32 = 1 << 7;
pub(super) const LCR_8N1: u32 = 0x03;

/// Enable the FIFOs and empty both.
pub(super) const FCR_RESET: u32 = 0x07;
pub(super) const MCR_DTR_RTS: u32 = 0x03;

// The mini-UART's own registers past the 16550 set: CNTL at 0x20 and BAUD
// at 0x28, which the four-byte stride makes indices 8 and 10.
pub(super) const AUX_CNTL: u64 = 8;
pub(super) const AUX_BAUD: u64 = 10;
pub(super) const CNTL_RX_TX: u32 = 0x03;
/// Its FCR slot is IIR, where bits 1 and 2 empty the FIFOs and bits 6 and 7
/// read back as set.
pub(super) const AUX_IIR_CLEAR: u32 = 0xC6;
