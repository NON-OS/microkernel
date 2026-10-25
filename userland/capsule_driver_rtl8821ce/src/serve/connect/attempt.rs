// NONOS Operating System (AGPL-3.0-or-later)

//! One candidate joined, and whether a failed join moves on to the next.

use nonos_wifi_core::address::Security;
use nonos_wifi_core::bss::{moves_on, Candidate};

use crate::assoc::Outcome;
use crate::fw::dma::Grant;
use crate::link::{RtlKeys, RtlLink};
use crate::phy::channel::{set_rf, Bw};
use crate::regs::Regs;

use super::install::answer;
use super::media::link_to_bss;
use super::ready::refused;
use super::request::ConnectRequest;
use super::result::ConnectResult;
use super::run::run;
use super::session::Session;

/// Join `c`: tuned to its channel, under the network's own address for the
/// security it offers, its beacon the join's first frame. With the answer
/// comes whether the access point ended the join, so the next may be tried
/// (`nonos_wifi_core::bss::moves_on`).
pub(super) fn attempt(
    c: &Candidate,
    req: &ConnectRequest<'_>,
    link: &mut RtlLink<Regs, Grant, Grant>,
    keys: &mut RtlKeys<Regs>,
    regs: &Regs,
    session: &mut Option<Session>,
) -> (ConnectResult, bool) {
    set_rf(regs, c.channel(), Bw::W20);
    let kind = Security::of_beacon(c.frame(), req.policy());
    let our_mac = match link.readdress(req.ssid, kind) {
        Ok(mac) => mac,
        Err(why) => return (refused(why), false),
    };
    link_to_bss(regs, c.frame());
    match run(link, req, our_mac, c.frame()) {
        Ok(report) => {
            let ended = !matches!(report.outcome, Outcome::Joined { .. });
            let next = ended && moves_on(report.failure);
            (answer(report, req.ssid, link, keys, regs, session), next)
        }
        Err(r) => (r, false),
    }
}
