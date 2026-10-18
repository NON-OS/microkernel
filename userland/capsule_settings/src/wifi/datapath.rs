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

//! Read the driver's data-path frame counts from the status op. These split a
//! stuck DHCP: no TX means net_core handed nothing down, RX frames without parsed
//! frames means the AP replies but the frames never decrypt.

/// The driver's TX and RX frame counts since bring-up, plus the number of
/// net_core link-protocol requests it has answered (zero means the stack never
/// reached the radio).
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct DataPath {
    pub tx_ok: u32,
    pub tx_drop: u32,
    pub rx_ring: u32,
    pub rx_eth: u32,
    pub netif_reqs: u32,
    pub rx_err: u32,
    /// The efuse control, address and LDO registers as the last stalled read left
    /// them. All zero means no read has stalled since boot, so a radio that
    /// reports the efuse stage with zeros here never entered the polling loop.
    pub efuse_ctl: u32,
    pub efuse_addr: u32,
    pub efuse_ldo: u32,
    /// The BAR the register window was taken from and the low half of its
    /// address. rtw88 uses bar_id 2 on this chip, so anything else means the
    /// registers are being read from the wrong window.
    pub bar_index: u32,
    pub window_va: u32,
    /// Where firmware bring-up stopped (0: it did not stop), the chunks that
    /// had landed, and `REG_MCUFW_CTRL` as the stop left it. Zero from a driver
    /// built before the report.
    pub fw_step: u8,
    pub fw_chunks: u32,
    pub fw_ctrl: u32,
}
