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

//! The control and status registers (CSR) and bus configuration registers
//! (BCR) the driver touches, per the Am79C970A and Am79C973 data sheets.

pub const CSR0: u16 = 0;
/// The init block's address, low then high sixteen bits.
pub const CSR1: u16 = 1;
pub const CSR2: u16 = 2;
pub const CSR3: u16 = 3;
pub const CSR4: u16 = 4;
/// The chip id, low then high half.
pub const CSR88: u16 = 88;
pub const CSR89: u16 = 89;
/// LED0, whose LEDOUT bit follows link status out of reset.
pub const BCR4: u16 = 4;
/// Software style: descriptor and init block layout.
pub const BCR20: u16 = 20;

pub const CSR0_INIT: u16 = 1 << 0;
pub const CSR0_STRT: u16 = 1 << 1;
pub const CSR0_STOP: u16 = 1 << 2;
pub const CSR0_TDMD: u16 = 1 << 3;
pub const CSR0_TXON: u16 = 1 << 4;
pub const CSR0_RXON: u16 = 1 << 5;
pub const CSR0_IDON: u16 = 1 << 8;

/// Every maskable cause masked. The driver polls and binds no line; with
/// INEA never set the part raises nothing anyway, and the masks keep a
/// stray INEA from a firmware that ran first from doing so either.
pub const CSR3_MASK_ALL: u16 = 0x5F00;
/// Linux's CSR4 for every PCI part: auto-pad on transmit, DMA-plus, and the
/// jabber and missed-frame counter overflow masks.
pub const CSR4_DEFAULT: u16 = 0x0915;
pub const BCR4_LEDOUT: u16 = 1 << 15;
/// SWSTYLE 2: 32-bit addresses and the 16-byte PCnet-PCI descriptors.
pub const SWSTYLE_PCNET_PCI: u16 = 2;

/// CSR88's low twelve bits on every AMD part; the part number sits above.
pub const CHIP_ID_AMD: u32 = 0x003;
