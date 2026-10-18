// NONOS Operating System (AGPL-3.0-or-later)

//! A join: the network found on the air, then joined.

use nonos_wifi_core::bss::Candidates;

use crate::fw::dma::Grant;
use crate::link::{RtlKeys, RtlLink};
use crate::regs::Regs;
use crate::status;

use super::attempt::attempt;
use super::hunt::find_candidates;
use super::ready::ready;
use super::request::parse_connect;
use super::result::{failure_code, ConnectResult, CODE_BAD_REQUEST, CODE_NOT_FOUND};
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
    // Hear the network's access points (or, hidden, their probe responses)
    // on the air, then join the best, and the next if the AP ends that join.
    let mut found = Candidates::new(req.ssid, req.policy());
    let beacons_heard = find_candidates(link, regs, req.ssid, probe_as, &mut found);
    let mut last = None;
    while let Some(c) = found.take_best() {
        let (r, next) = attempt(&c, &req, link, keys, regs, session);
        if !next {
            return r;
        }
        last = Some(r);
    }
    if let Some(r) = last {
        return r;
    }
    // recv carries how many beacons of any network were heard during the
    // hunt: 0 means the radio heard nothing (receive/tuning problem), a
    // non-zero count means beacons arrive but the target's did not.
    let recv = beacons_heard;
    let Some(why) = found.refusal() else {
        status::debug(b"[rtl8821ce] connect: network not found\n");
        return ConnectResult { recv, ..ConnectResult::code(CODE_NOT_FOUND) };
    };
    // Heard, but not a network this station can join as it claims to be.
    let (code, ap_code) = failure_code(why);
    ConnectResult { recv, ap_code, ..ConnectResult::code(code) }
}
