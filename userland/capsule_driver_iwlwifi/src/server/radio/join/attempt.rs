// NONOS Operating System (AGPL-3.0-or-later)

//! The candidates joined in turn: the best first, and the next if its access
//! point ended the join, each join under fresh randomness.

use crate::firmware::gen3::join::address::join_as;
use crate::firmware::gen3::join::exchange::Progress;
use crate::firmware::gen3::join::fw::Fw;
use crate::firmware::gen3::join::run::JoinEnd;
use crate::server::join_wire::{ConnectRequest, ConnectResult, CODE_NO_ENTROPY};
use nonos_wifi_core::address::Security;
use nonos_wifi_core::bss::{moves_on, Candidates};
use nonos_wifi_core::mlme::{Entropy, JoinRequest};
use nonos_wifi_core::rsn::JoinPolicy;

use super::super::bring::Up;
use super::super::clock::Uptime;
use super::super::entropy::draw_entropy;
use super::super::grant::Grant;
use super::say::say_join;
use super::state::Join;
use super::transmit::recorded;

/// Join `found`'s candidates in turn; the answer of the last join run.
pub(super) fn try_each(
    up: &mut Up,
    join: &mut Join,
    tx: &'static Grant,
    found: &mut Candidates,
    req: &ConnectRequest<'_>,
    policy: JoinPolicy,
    first: Entropy,
) -> ConnectResult {
    let (mut first, mut last, mut clock) = (Some(first), None, Uptime);
    while let Some(c) = found.take_best() {
        let Some(entropy) = first.take().or_else(draw_entropy) else { break };
        // The network's own address: its name under the security the join runs.
        let kind = Security::of_beacon(c.frame(), policy);
        let Some(our_mac) = join.address(req.ssid, kind) else { break };
        let jr = JoinRequest { our_mac, ssid: req.ssid, passphrase: req.pass, policy, entropy };
        let mut fw =
            Fw { dev: &mut up.dev, tx, clock: &mut clock, q: &mut join.q, inbox: &mut join.inbox };
        let mut progress = Progress::default();
        let out =
            join_as(&mut fw, &jr, c.frame(), c.channel(), up.tx_ant, up.rx_ant, &mut progress);
        say_join(&progress, &out);
        let next = out.as_ref().is_err_and(next_after);
        last = Some(recorded(join, req.ssid, out, progress));
        if !next {
            break;
        }
    }
    last.unwrap_or(ConnectResult::code(CODE_NO_ENTROPY))
}

/// Whether the access point ended the join (`nonos_wifi_core::bss::moves_on`):
/// a frame that was not the network's, a refusal, silence. A setup, firmware
/// or key failure is this side's, and ends the request.
fn next_after(end: &JoinEnd) -> bool {
    match end {
        JoinEnd::NotTheNetwork => true,
        JoinEnd::TimedOut => moves_on(None),
        JoinEnd::Refused(f) => moves_on(Some(*f)),
        JoinEnd::Setup(_) | JoinEnd::Firmware(_) | JoinEnd::Keys(_) => false,
    }
}
