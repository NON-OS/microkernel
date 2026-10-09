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

//! Finding the network on the air before a join. The hunt listens: beacons
//! name every network that broadcasts its name, so nothing is transmitted and
//! no saved network's name leaves the machine. Only for a network the person
//! marked hidden (its beacons carry no name) does the hunt send a probe
//! request naming it, once per channel, from the per-boot random address; the
//! probe response that answers it carries the name and is taken like a beacon.

use nonos_wifi_core::dot11::parse::parse_beacon;

use crate::fw::dma::Grant;
use crate::link::RtlLink;
use crate::phy::channel::{set_rf, Bw};
use crate::regs::Regs;

use super::super::{SCAN_CHANNELS, SCAN_FRAME_MAX};
use super::probe::hunt_probe;

/// Milliseconds of uptime to dwell on each channel while hunting the target's
/// beacon: two and a half beacon intervals (102.4 ms each), so one beacon
/// lost to noise or a busy medium does not hide the network. At 120 ms one
/// miss read as "not heard on any channel" on the HP.
const BEACON_HUNT_DWELL_MS: i64 = 250;
/// Full sweeps before the network is called not heard.
const HUNT_SWEEPS: usize = 2;

/*
 * Hunt the channels for a beacon matching `ssid`, copying the raw frame into `out`
 * and returning its length and the channel it was heard on. The join needs the
 * real beacon (not the scan summary) because the state machine reads the BSSID,
 * channel and RSN element straight out of it.
 * Returns the matched beacon (length and channel) if found, together with the
 * number of beacons of ANY network heard during the hunt. That count turns a
 * bare "not found" into a diagnosis: zero means the radio heard nothing at all
 * on any channel (a receive-path or tuning problem), non-zero means beacons are
 * arriving but the target's was not among them (wrong SSID or it was off-air).
 */
pub(super) fn find_beacon(
    link: &mut RtlLink<Regs, Grant, Grant>,
    regs: &Regs,
    ssid: &[u8],
    hidden: bool,
    our_mac: [u8; 6],
    out: &mut [u8; SCAN_FRAME_MAX],
) -> (Option<(usize, u8)>, u32) {
    let mut frame = [0u8; SCAN_FRAME_MAX];
    let mut beacons = 0u32;
    let sweeps = SCAN_CHANNELS.iter().enumerate().cycle().take(SCAN_CHANNELS.len() * HUNT_SWEEPS);
    for (i, &ch) in sweeps {
        set_rf(regs, ch, Bw::W20);
        let mut probe = [0u8; 64];
        if let Some(n) = hunt_probe(&mut probe, hidden, our_mac, ssid, i) {
            link.send_raw(&probe[..n]);
        }
        let leave_at = nonos_libc::mk_uptime_ms() + BEACON_HUNT_DWELL_MS;
        while nonos_libc::mk_uptime_ms() < leave_at {
            let Some(n) = link.poll_raw(&mut frame) else {
                continue;
            };
            if let Some(info) = parse_beacon(&frame[..n]) {
                beacons += 1;
                if info.ssid == ssid {
                    let channel =
                        if info.channel >= 1 && info.channel <= 14 { info.channel } else { ch };
                    out[..n].copy_from_slice(&frame[..n]);
                    return (Some((n, channel)), beacons);
                }
            }
        }
    }
    (None, beacons)
}
