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

//! The registers and bits the per-version steps use, as Linux names them in
//! `enum rtl8168_8101_registers` and `enum rtl8168_registers`.

/// Config2 (bit 0: the PCI bus runs at 66 MHz).
pub const REG_CONFIG2: usize = 0x53;
/// CPlusCmd.
pub const REG_CPLUS_CMD: usize = 0xE0;
/// 0xEC is EarlyTxThres on the 8169 and MaxTxPacketSize on the 8168/810x.
pub const REG_EARLY_TX_THRES: usize = 0xEC;
pub const REG_MAX_TX_PACKET_SIZE: usize = 0xEC;
/// MAR0: the 64-bit multicast hash filter, two dwords.
pub const REG_MAR0: usize = 0x08;
/// ChipCmd StopReq: ask the MAC to stop DMA before a reset.
pub const CMD_STOP_REQ: u8 = 0x80;
/// TxConfig TXCFG_EMPTY (8111e-vl and later): the TX FIFO has drained.
pub const TXCFG_EMPTY: u32 = 1 << 11;
/// IntrMitigate; on the 8125B and later its low bits report FIFO state.
pub const REG_INTR_MITIGATE: usize = 0xE2;
pub const INTR_MITIGATE_RXTX_EMPTY: u16 = 0x0103;
/// MCU (8168g and later) and the FIFO-empty bits in it.
pub const REG_MCU: usize = 0xD3;
pub const MCU_RXTX_EMPTY: u8 = (1 << 5) | (1 << 4);
pub const MCU_NOW_IS_OOB: u8 = 1 << 7;
pub const MCU_LINK_LIST_RDY: u8 = 1 << 1;
/// MISC (8168e and later) and the RX data-valid gate in it.
pub const REG_MISC: usize = 0xF0;
pub const MISC_RXDV_GATED_EN: u32 = 1 << 19;
/// RxConfig accept bits, cleared by rtl_rx_close.
pub const RX_CONFIG_ACCEPT_MASK: u32 = 0x3F;
