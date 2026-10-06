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

//! How far a proxy's network has got, as it says when asked.
//!
//! While net.anon bootstrapped, for minutes on a cold boot, the browser
//! showed "Connecting to" and then failed the page with "not connected yet".
//! Both proxies now answer a status ask (capsule_socks5 server/request.rs,
//! capsule_net_anon server/socks/frame.rs) with the step their network has
//! reached, and the status line says it while the page waits.
//!
//! The ask is one byte, 6, and touches no conversation. The answer is five:
//! 3, the format version 1, whether a stream can be opened now, the step,
//! and the number of steps. net.socks5 adds a sixth: how many exits it has
//! walked away from this session because they did not answer. A proxy older than the ask answers it as
//! something else (a reset of stream 0, or a close), which reads here as no
//! answer at all; nothing else changes for it.
//!
//! Pure; the proofs hold it.

use alloc::format;
use alloc::string::String;

use super::choice::Network;

/// The status ask.
pub const STATUS_ASK: u8 = 6;

/// The marker of a status answer.
const STATUS: u8 = 3;

/// The only format there is.
const VERSION: u8 = 1;

/// More steps than either proxy counts is not an answer.
const STEPS_MAX: u8 = 8;

/// How far a network has got.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Progress {
    pub ready: bool,
    pub step: u8,
    pub steps: u8,
    /// Exits net.socks5 walked away from this session for silence, when it
    /// says; net.anon does not.
    pub silent: Option<u8>,
}

/// What asking a proxy how far it has got came to.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Heard {
    /// It said.
    Known(Progress),
    /// It did not answer in the wait: it is in a long call of its own, as
    /// net.socks5 is while it opens its mixnet session.
    Busy,
    /// It answered with something else: a proxy older than the ask.
    Unknown,
}

/// The status answer in `raw`, or `None` for bytes that are not one.
pub fn read(raw: &[u8]) -> Option<Progress> {
    let (head, silent) = match raw.len() {
        5 => (raw, None),
        6 => (&raw[..5], Some(raw[5])),
        _ => return None,
    };
    let &[STATUS, VERSION, ready, step, steps] = head else { return None };
    let sane = ready <= 1 && (1..=STEPS_MAX).contains(&steps) && (1..=steps).contains(&step);
    sane.then_some(Progress { ready: ready == 1, step, steps, silent })
}

/// What the network calls itself in a sentence.
fn name(net: Network) -> &'static str {
    match net {
        Network::Nym => "Nym",
        Network::Anyone => "Anyone",
        Network::Direct => "The direct network",
    }
}

/// What the network is doing at `step`, as each proxy counts its steps.
fn doing(net: Network, step: u8) -> &'static str {
    match (net, step) {
        (Network::Anyone, 1) => "fetching the directory authorities' keys",
        (Network::Anyone, 2) => "fetching the list of relays",
        (Network::Anyone, 3) => "fetching the relays' descriptors",
        (Network::Anyone, 4) => "connecting to its guard relay",
        (Network::Anyone, _) => "building its circuit",
        (Network::Nym, 1) => "waiting for net.nym to start",
        (Network::Nym, 3) => "trying another exit",
        (_, _) => "opening its mixnet session",
    }
}

/// The status line's words for a network that is not ready, or `None` when
/// it said nothing that helps: ready, or not asked, or too old to say.
pub fn still(net: Network, heard: Option<Heard>) -> Option<String> {
    match heard? {
        /* An exit did not answer and net.socks5 walked on to the next. */
        Heard::Known(p) if net == Network::Nym && p.step == 3 => Some(format!(
            "Nym: exit did not answer, trying another ({})",
            p.silent.unwrap_or(1)
        )),
        Heard::Known(p) if !p.ready => Some(format!(
            "{} is still {} (step {} of {})",
            name(net),
            doing(net, p.step),
            p.step,
            p.steps
        )),
        Heard::Busy if net == Network::Nym => {
            Some(String::from("net.socks5 is busy, most likely opening its mixnet session"))
        }
        Heard::Busy => Some(format!("{} is busy and has not answered yet", name(net))),
        Heard::Known(_) | Heard::Unknown => None,
    }
}
