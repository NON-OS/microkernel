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

//! The join run under the request's policy.

use nonos_wifi_core::mlme::JoinRequest;

use crate::assoc;
use crate::fw::dma::Grant;
use crate::link::RtlLink;
use crate::regs::Regs;
use crate::status;

use super::consts::*;
use super::entropy::draw_entropy;
use super::pace::Uptime;
use super::progress::log_progress;
use super::request::ConnectRequest;
use super::result::{ConnectResult, CODE_NO_ENTROPY};

/// Run the join on the network's beacon, under the request's policy.
pub(super) fn run(
    link: &mut RtlLink<Regs, Grant, Grant>,
    req: &ConnectRequest<'_>,
    our_mac: [u8; 6],
    beacon: &[u8],
) -> Result<assoc::Report, ConnectResult> {
    // The SNonce goes out in message 2; SAE's rand, mask and stand-in password
    // are drawn separately so nothing secret is derived from it.
    let Some(entropy) = draw_entropy() else {
        return Err(ConnectResult::code(CODE_NO_ENTROPY));
    };
    let policy = req.policy();
    let join = JoinRequest { our_mac, ssid: req.ssid, passphrase: req.pass, policy, entropy };
    status::debug(b"[rtl8821ce] connect: joining\n");
    /*
     * A network that offers WPA3 and WPA2 side by side is joined with WPA3,
     * and a WPA3 join that does not finish is not tried again with WPA2:
     * anyone who can jam the SAE exchange would otherwise get a WPA2
     * handshake to guess the passphrase from offline.
     */
    let report = assoc::run_join(link, &join, beacon, JOIN_BUDGET_MS, &mut Uptime);
    log_progress(&report);
    Ok(report)
}
