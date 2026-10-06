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

//! The class requests an NCM function takes (NCM 1.0, 6.2, table 6-2, as
//! Linux include/uapi/linux/usb/cdc.h numbers them), all sent to the
//! communications interface, and the capability bits that gate them.

/// CDC ECM 1.2, 6.2.4; cdc_ncm_info sends it through its set_rx_mode,
/// usbnet_cdc_update_filter.
pub const SET_ETHERNET_PACKET_FILTER: u8 = 0x43;
pub const GET_NTB_PARAMETERS: u8 = 0x80;
pub const SET_NTB_FORMAT: u8 = 0x84;
pub const SET_NTB_INPUT_SIZE: u8 = 0x86;
pub const GET_MAX_DATAGRAM_SIZE: u8 = 0x87;
pub const SET_MAX_DATAGRAM_SIZE: u8 = 0x88;
pub const SET_CRC_MODE: u8 = 0x8A;

/// SET_NTB_FORMAT's wValue for 16-bit NTBs (NCM 1.0, 6.2.5).
pub const NTB16_FORMAT: u16 = 0x0000;
/// SET_CRC_MODE's wValue: datagrams without a CRC (NCM 1.0, 6.2.11).
pub const CRC_NOT_APPENDED: u16 = 0x0000;

/// bmNetworkCapabilities of the NCM functional descriptor (NCM 1.0,
/// 5.2.1, table 5-2; Linux USB_CDC_NCM_NCAP_*).
pub const CAP_MAX_DATAGRAM_SIZE: u8 = 1 << 3;
pub const CAP_CRC_MODE: u8 = 1 << 4;
/// The device takes the 8-byte form of SET_NTB_INPUT_SIZE.
pub const CAP_NTB_INPUT_SIZE_8: u8 = 1 << 5;

/// The packet filter usbnet_cdc_update_filter sets with IFF_ALLMULTI:
/// directed, broadcast and all multicast (CDC ECM 1.2, table 8).
pub const FILTER: u16 = 0x0004 | 0x0008 | 0x0002;
