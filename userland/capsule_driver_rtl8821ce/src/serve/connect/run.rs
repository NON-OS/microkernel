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
use nonos_wifi_core::rsn::JoinPolicy;

use crate::assoc;
use crate::fw::dma::Grant;
use crate::link::RtlLink;
use crate::regs::Regs;
use crate::status;

use super::consts::*;
use super::entropy::{draw_entropy, retry_with_psk};
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
    let policy = if req.wpa3_only { JoinPolicy::WPA3_ONLY } else { JoinPolicy::ANY };
    let join = JoinRequest { our_mac, ssid: req.ssid, passphrase: req.pass, policy, entropy };
    status::debug(b"[rtl8821ce] connect: joining\n");
    let mut now_ms = || nonos_libc::mk_uptime_ms() as u64;
    let mut report = assoc::run_join(link, &join, beacon, JOIN_BUDGET_MS, &mut now_ms);
    log_progress(&report);
    /*
     * A network that offers WPA3 and WPA2 side by side (transition mode, as
     * many home routers ship) is joined with WPA3 first. When that does not
     * finish, for any reason but the router saying the password is wrong, it
     * is joined again with WPA2, which the router offers and which every WPA2
     * station uses there. Never for a network saved as WPA3.
     */
    if retry_with_psk(&report, req.wpa3_only) {
        let Some(entropy) = draw_entropy() else {
            return Err(ConnectResult::code(CODE_NO_ENTROPY));
        };
        let policy = JoinPolicy::PSK_ONLY;
        let psk = JoinRequest { our_mac, ssid: req.ssid, passphrase: req.pass, policy, entropy };
        status::line(b"[rtl8821ce] connect: WPA3 did not finish; joining with WPA2\n");
        report = assoc::run_join(link, &psk, beacon, PSK_BUDGET_MS, &mut now_ms);
        log_progress(&report);
    }
    Ok(report)
}
