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

//! USB-block registers and their bits, as r8152.c defines them.

pub const USB_CTRL: u16 = 0xd406;
pub const RX_AGG_DISABLE: u16 = 0x0010;
pub const RX_ZERO_EN: u16 = 0x0080;

pub const RX_BUF_TH: u16 = 0xd40c;
/// RX_THR_B, the RTL8153B threshold rtl8153b_up writes.
pub const RX_THR_B: u32 = 0x0001_0001;

pub const UPT_RXDMA_OWN: u16 = 0xd437;
pub const OWN_UPDATE: u8 = 1 << 0;
pub const OWN_CLEAR: u8 = 1 << 1;

pub const BMU_RESET: u16 = 0xd4b0;
pub const BMU_RESET_EP_IN: u8 = 0x01;
pub const BMU_RESET_EP_OUT: u8 = 0x02;

pub const FW_TASK: u16 = 0xd4e8;
pub const FC_PATCH_TASK: u16 = 1 << 1;
