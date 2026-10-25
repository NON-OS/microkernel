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

pub(super) const RBR: u64 = 0;
pub(super) const THR: u64 = 0;
pub(super) const IER: u64 = 1;
pub(super) const FCR: u64 = 2;
pub(super) const LCR: u64 = 3;
pub(super) const MCR: u64 = 4;
pub(super) const LSR: u64 = 5;
pub(super) const DLL: u64 = 0;
pub(super) const DLM: u64 = 1;

pub(super) const LSR_DR: u8 = 1 << 0;
pub(super) const LSR_THRE: u8 = 1 << 5;
pub(super) const LSR_TEMT: u8 = 1 << 6;

pub(super) const LCR_DLAB: u8 = 1 << 7;
pub(super) const LCR_8N1: u8 = 0x03;

pub(super) const FCR_ENABLE: u8 = 1 << 0;
pub(super) const FCR_CLEAR_RX: u8 = 1 << 1;
pub(super) const FCR_CLEAR_TX: u8 = 1 << 2;
pub(super) const FCR_TRIGGER_14: u8 = 0xC0;

pub(super) const IER_RDA: u8 = 1 << 0;

pub(super) const MCR_DTR: u8 = 1 << 0;
pub(super) const MCR_RTS: u8 = 1 << 1;
pub(super) const MCR_OUT2: u8 = 1 << 3;
