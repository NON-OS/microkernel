/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

//! The status op's reply: the bring-up stage and the data-path diagnostics.

use super::super::radio::Radio;
use super::super::Stage;
use super::WIFI_HDR;

/*
 * The bring-up stage byte, then the data-path counts as five LE u32s: TX
 * enqueued, TX dropped, RX frames the ring returned, RX frames parsed to
 * Ethernet, and net_core link-protocol requests answered. netif=0 means the
 * stack never reached the radio; TX=0 with netif>0 means it probed but never
 * bound; RX-ring without RX-eth means frames arrive but never decrypt.
 */
pub(super) fn status_reply(stage: Stage, radio: &Radio, out: &mut [u8]) -> Option<usize> {
    out[WIFI_HDR] = stage as u8;
    let stats = match radio {
        Radio::Up(up) => up.link.stats(),
        Radio::Down => crate::link::LinkStats::default(),
    };
    out[WIFI_HDR + 1..WIFI_HDR + 5].copy_from_slice(&stats.tx_ok.to_le_bytes());
    out[WIFI_HDR + 5..WIFI_HDR + 9].copy_from_slice(&stats.tx_drop.to_le_bytes());
    out[WIFI_HDR + 9..WIFI_HDR + 13].copy_from_slice(&stats.rx_ring.to_le_bytes());
    out[WIFI_HDR + 13..WIFI_HDR + 17].copy_from_slice(&stats.rx_eth.to_le_bytes());
    out[WIFI_HDR + 17..WIFI_HDR + 21].copy_from_slice(&stats.netif_reqs.to_le_bytes());
    out[WIFI_HDR + 21..WIFI_HDR + 25].copy_from_slice(&stats.rx_err.to_le_bytes());
    /*
     * Then the efuse registers as the last stalled read left them. A bring-up that
     * stops at the efuse says nothing about why on a machine with no serial
     * console; these three separate a dead register window from a live one whose
     * efuse controller is never clocked.
     */
    let (ctl, addr, ldo) = crate::efuse::diag();
    out[WIFI_HDR + 25..WIFI_HDR + 29].copy_from_slice(&ctl.to_le_bytes());
    out[WIFI_HDR + 29..WIFI_HDR + 33].copy_from_slice(&addr.to_le_bytes());
    out[WIFI_HDR + 33..WIFI_HDR + 37].copy_from_slice(&ldo.to_le_bytes());
    /*
     * And which BAR the window came from. rtw88 hardcodes bar_id 2 for this chip
     * while this driver takes the first MMIO BAR the broker reports, so an index
     * other than 2 means the registers are being read somewhere they do not live.
     */
    let (bar, va) = crate::setup::window();
    out[WIFI_HDR + 37..WIFI_HDR + 41].copy_from_slice(&bar.to_le_bytes());
    out[WIFI_HDR + 41..WIFI_HDR + 45].copy_from_slice(&va.to_le_bytes());
    Some(WIFI_HDR + 45)
}
