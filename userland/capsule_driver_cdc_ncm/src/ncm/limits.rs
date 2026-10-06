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

//! The NTB sizes in each direction, as Linux cdc_ncm_check_rx_max,
//! cdc_ncm_check_tx_max and cdc_ncm_update_rxtx_max derive them, with
//! driver.xhci0's BULK_MAX in place of Linux's 16384-byte default.

use nonos_usbnet::nic::ETH_FRAME_MAX;
use nonos_usbnet::xhci::BULK_MAX;

use super::ntb_out::NTH16_LEN;

/// USB_CDC_NCM_NTB_MIN_IN_SIZE and _OUT_SIZE (NCM 1.0, 6.2.7).
const NTB_MIN: usize = 2048;
/// CDC_NCM_NTB_MAX_SIZE_RX and _TX.
const NTB_MAX: usize = 65536;
/// CDC_NCM_MIN_TX_PKT.
const MIN_TX_PKT: usize = 512;
/// cdc_ncm_init's max_ndp_size: an NDP16 with room for
/// CDC_NCM_DPT_DATAGRAMS_MAX (40) entries and the terminating one.
const MAX_NDP16: usize = 8 + (40 + 1) * 4;
/// Linux's least tx_max: a full frame, that NDP and the NTH16. Here the
/// NDP is 16 bytes, and the rest is room for the datagram's alignment.
const ONE_FRAME_NTB: usize = ETH_FRAME_MAX + MAX_NDP16 + NTH16_LEN;

/// The NTB input size asked of the device: at most one bulk transfer and
/// what it offers, and never under 2048 bytes, even when it offers less.
pub fn rx_max(in_max: u32) -> usize {
    let offered = in_max as usize;
    let max = offered.clamp(NTB_MIN, NTB_MAX);
    offered.min(BULK_MAX).clamp(NTB_MIN, max)
}

/// The largest NTB sent. Linux makes it one byte longer when it falls on a
/// packet boundary, so a block padded to it ends in a short packet; at
/// BULK_MAX it is made one byte shorter instead.
pub fn tx_max(out_max: u32, mps: usize) -> usize {
    let offered = out_max as usize;
    let max = if offered == 0 { NTB_MAX } else { offered.clamp(NTB_MIN, NTB_MAX) };
    let val = offered.min(BULK_MAX).clamp(ONE_FRAME_NTB.min(max), max).min(BULK_MAX);
    match (val != offered && ends_on_packet(val, mps), val < BULK_MAX) {
        (true, true) => val + 1,
        (true, false) => val - 1,
        _ => val,
    }
}

/// Linux's min_tx_pkt: a block longer than this is padded to tx_max, so
/// padding never costs more than three packets.
pub fn min_tx_pkt(tx_max: usize, mps: usize) -> usize {
    tx_max.saturating_sub(3 * mps).max(MIN_TX_PKT).min(tx_max)
}

/// A length that fills its last packet. A multiple of 64 counts too, as in
/// the ECM driver: driver.xhci0 moves a high-speed or SuperSpeed pipe in
/// 512 or 1024-byte packets whatever the descriptor says.
pub fn ends_on_packet(len: usize, mps: usize) -> bool {
    len.is_multiple_of(64) || (mps != 0 && len.is_multiple_of(mps))
}
