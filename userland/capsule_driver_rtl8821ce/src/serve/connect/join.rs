// NONOS Operating System (AGPL-3.0-or-later)

//! A join: the network found on the air, then joined.

use nonos_wifi_core::address::Security;

use crate::fw::dma::Grant;
use crate::link::{RtlKeys, RtlLink};
use crate::phy::channel::{set_rf, Bw};
use crate::regs::Regs;
use crate::status;

use super::super::SCAN_FRAME_MAX;
use super::hunt::find_beacon;
use super::install::answer;
use super::media::link_to_bss;
use super::ready::{ready, refused};
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
    let probe_as = match ready(link, &req) {
        Ok(mac) => mac,
        Err(r) => return r,
    };
    // Find the network's beacon (or, hidden, its probe response) on the air,
    // which also tells us its channel.
    let mut beacon = [0u8; SCAN_FRAME_MAX];
    let (found, beacons_heard) = find_beacon(link, regs, req.ssid, probe_as, &mut beacon);
    let Some((beacon_len, channel)) = found else {
        status::debug(b"[rtl8821ce] connect: network not found\n");
        // recv carries how many beacons of any network were heard during the
        // hunt: 0 means the radio heard nothing (receive/tuning problem), a
        // non-zero count means beacons arrive but the target's did not.
        return ConnectResult { recv: beacons_heard, ..ConnectResult::code(CODE_NOT_FOUND) };
    };
    set_rf(regs, channel, Bw::W20);
    // The network's own address: its name under the security the join runs.
    let kind = Security::of_beacon(&beacon[..beacon_len], req.policy());
    let our_mac = match link.readdress(req.ssid, kind) {
        Ok(mac) => mac,
        Err(why) => return refused(why),
    };

    link_to_bss(regs, &beacon);

    let report = match run(link, &req, our_mac, &beacon[..beacon_len]) {
        Ok(report) => report,
        Err(r) => return r,
    };
    answer(report, req.ssid, link, keys, regs, session)
}
