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


//! Short names in the manager: resolving one when a stream is opened to it,
//! and fetching the signed list from a DNS service when one is wanted.
//!
//! The list is fetched only when a caller has asked for a short name, over
//! an onion stream to one of the six DNS services, as the fork fetches it
//! (anyone_hosts_update.c). A caller asking before a current list is in
//! hand gets E_NAMES_PENDING and asks again; nothing waits inside a tick. A
//! failed fetch moves to the next service and waits RETRY_SECONDS. No name
//! is written to the log: the log says that a name resolved, not which.

extern crate alloc;

use alloc::vec::Vec;

use crate::onion::address::encode;
use crate::onion::fetch::{body_within, Answer};
use crate::onion::names::{defaults, may_replace, normal, verify, NameList, Pins, Resolve, FETCH_PATH, LIST_MAX};
use crate::stream::{StreamStage, ONION};
use crate::trace;

use super::out::{open_stream, send_data, SendError};
use super::state::Manager;

/// A fetch gets this long, its onion lookup included.
const FETCH_SECONDS: u64 = 120;
/// After a failed fetch, the next waits this long.
const RETRY_SECONDS: u64 = 300;
/// A current list is fetched again after this long, when names are in use,
/// so a name added since is found.
const REFRESH_SECONDS: u64 = 6 * 3600;

pub struct NameState {
    builtin: NameList,
    /// The keys a list may be signed with, and the services it is fetched
    /// from: the six hardcoded services in both. Crate-visible so the
    /// end-to-end proofs can stand a simulated service in for them; nothing
    /// in the capsule changes them.
    pub(crate) signers: Vec<[u8; 32]>,
    pub(crate) services: Vec<[u8; 32]>,
    pub(crate) list: Option<NameList>,
    fetched_at: u64,
    pins: Pins,
    fetch: Option<Fetch>,
    wanted: bool,
    next_try: u64,
    target: usize,
}

struct Fetch {
    stream: u16,
    sent: bool,
    deadline: u64,
}

impl Default for NameState {
    fn default() -> Self {
        Self {
            builtin: defaults::list(),
            signers: defaults::signers(),
            services: defaults::signers(),
            list: None,
            fetched_at: 0,
            pins: Pins::default(),
            fetch: None,
            wanted: false,
            next_try: 0,
            target: 0,
        }
    }
}

/// The identity key the short name `host` names, or why not.
pub fn resolve(state: &mut Manager, host: &[u8], now: u64) -> Result<[u8; 32], SendError> {
    let name = normal(host);
    let n = &mut state.names;
    match n.pins.resolve(&name, &n.builtin, n.list.as_ref(), now) {
        Ok(id) => {
            trace::say(b"onion short name resolved by the signed list");
            Ok(id)
        }
        Err(Resolve::NotAName) => Err(SendError::BadOnion),
        Err(Resolve::Pending) => {
            n.wanted = true;
            Err(SendError::NamesPending)
        }
        Err(Resolve::Unknown) => {
            n.wanted = true;
            Err(SendError::NameUnknown)
        }
        Err(Resolve::Changed) => {
            trace::say(b"onion short name now names another service than earlier this boot, refused");
            Err(SendError::NameChanged)
        }
        Err(Resolve::Full) => Err(SendError::TableFull),
    }
}

/// Advance a fetch, or start one when a list is wanted and due.
pub fn tick(state: &mut Manager, now: u64) {
    let Some(fetch) = state.names.fetch.as_mut() else {
        if due(&state.names, now) {
            start(state, now);
        }
        return;
    };
    let id = fetch.stream;
    let late = now >= fetch.deadline;
    let Some(stream) = state.streams.iter().find(|s| s.id == id) else {
        return failed(state, now, b"names fetch stream gone");
    };
    match stream.stage {
        StreamStage::Opening if late => failed(state, now, b"names fetch did not open in time"),
        StreamStage::Opening => {}
        StreamStage::Open if !fetch.sent => {
            let request = request();
            if send_data(state, id, &request).is_err() {
                return failed(state, now, b"names fetch request not sent");
            }
            if let Some(f) = state.names.fetch.as_mut() {
                f.sent = true;
            }
        }
        StreamStage::Open if late || stream.inbound.len() > LIST_MAX + 4096 => failed(state, now, b"names fetch too slow or too large"),
        StreamStage::Open => {}
        StreamStage::Ended(_) => finished(state, now),
    }
}

fn due(n: &NameState, now: u64) -> bool {
    let stale = match &n.list {
        None => true,
        Some(l) => now > l.valid_until || now >= n.fetched_at.saturating_add(REFRESH_SECONDS),
    };
    n.wanted && stale && now >= n.next_try
}

fn request() -> Vec<u8> {
    let mut out = Vec::with_capacity(64);
    out.extend_from_slice(b"GET ");
    out.extend_from_slice(FETCH_PATH.as_bytes());
    out.extend_from_slice(b" HTTP/1.0\r\n\r\n");
    out
}

fn start(state: &mut Manager, now: u64) {
    let Some(service) = state.names.services.get(state.names.target % state.names.services.len().max(1)).copied() else {
        return;
    };
    let address = encode(&service);
    match open_stream(state, &address, 80, now, ONION) {
        Ok(stream) => {
            trace::say_num(b"names fetch started from dns service", state.names.target as u64);
            state.names.fetch = Some(Fetch { stream, sent: false, deadline: now.saturating_add(FETCH_SECONDS) });
        }
        Err(_) => {
            state.names.next_try = now.saturating_add(RETRY_SECONDS);
        }
    }
}

fn finished(state: &mut Manager, now: u64) {
    let Some(fetch) = state.names.fetch.take() else { return };
    let raw = state.streams.iter_mut().find(|s| s.id == fetch.stream).map(|s| core::mem::take(&mut s.inbound)).unwrap_or_default();
    state.streams.retain(|s| s.id != fetch.stream);
    let body = match body_within(&raw, LIST_MAX) {
        Ok(body) => body,
        Err(Answer::NotFound) => return failed(state, now, b"names list not found at the dns service"),
        Err(_) => return failed(state, now, b"names list answer not usable"),
    };
    match verify(body, &state.names.signers, now) {
        Ok(list) if may_replace(state.names.list.as_ref(), &list) => {
            trace::say_num(b"names list verified, names", list.entries.len() as u64);
            state.names.list = Some(list);
            state.names.fetched_at = now;
            state.names.wanted = false;
        }
        Ok(_) => failed(state, now, b"names list refused: older than the one held"),
        Err(why) => failed(state, now, why.said()),
    }
}

fn failed(state: &mut Manager, now: u64, why: &[u8]) {
    trace::say(why);
    if let Some(fetch) = state.names.fetch.take() {
        state.streams.retain(|s| s.id != fetch.stream);
    }
    state.names.target = (state.names.target + 1) % state.names.services.len().max(1);
    state.names.next_try = now.saturating_add(RETRY_SECONDS);
}
