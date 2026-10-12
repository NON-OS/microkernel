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

//! A join: the network found on the air, then joined.

use crate::fw::dma::Grant;
use crate::link::{RtlKeys, RtlLink};
use crate::phy::channel::{set_rf, Bw};
use crate::regs::Regs;
use crate::status;

use super::super::radio::read_mac;
use super::super::SCAN_FRAME_MAX;
use super::hunt::find_beacon;
use super::install::answer;
use super::media::link_to_bss;
use super::request::parse_connect;
use super::result::{ConnectResult, CODE_BAD_REQUEST, CODE_NOT_FOUND};
use super::run::run;
use super::session::Session;

/// Join the network named in the request. The body is the SSID, passphrase and
/// optional flags: `[ssid_len][ssid][pass_len][pass][flags]` (flags bit 0: the
/// network was saved as WPA3, refuse WPA2; bit 1: it is hidden). Returns 0 on
/// success, or a negative code naming where it stopped, with the handshake's
/// progress, the AKM that ran and the AP's own status or reason code.
pub(in crate::serve) fn connect(
    body: &[u8],
    link: &mut RtlLink<Regs, Grant, Grant>,
    keys: &mut RtlKeys<Regs>,
    regs: &Regs,
    session: &mut Option<Session>,
) -> ConnectResult {
    let Some(req) = parse_connect(body) else {
        return ConnectResult::code(CODE_BAD_REQUEST);
    };
    let our_mac = read_mac(regs);
    // Find the network's beacon (or, hidden, its probe response) on the air,
    // which also tells us its channel.
    let mut beacon = [0u8; SCAN_FRAME_MAX];
    let (found, beacons_heard) = find_beacon(link, regs, req.ssid, req.hidden, our_mac, &mut beacon);
    let Some((beacon_len, channel)) = found else {
        status::debug(b"[rtl8821ce] connect: network not found\n");
        // recv carries how many beacons of any network were heard during the
        // hunt: 0 means the radio heard nothing (receive/tuning problem), a
        // non-zero count means beacons arrive but the target's did not.
        return ConnectResult { recv: beacons_heard, ..ConnectResult::code(CODE_NOT_FOUND) };
    };
    set_rf(regs, channel, Bw::W20);

    link_to_bss(regs, &beacon);

    let report = match run(link, &req, our_mac, &beacon[..beacon_len]) {
        Ok(report) => report,
        Err(r) => return r,
    };
    answer(report, req.ssid, link, keys, regs, session)
}
