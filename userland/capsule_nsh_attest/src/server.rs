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

//! The service loop. One caller is answered, the measured nsh.front; any
//! other sender is refused. Between requests, until the host keys are
//! held and kept and the enrolment request is written, the loop wakes every
//! WAKE_MS to make progress on them, so the keys are made at first boot
//! whether or not anyone connects; after that it waits for requests alone.

use crate::attest::{document, entries};
use crate::caller::is_front;
use crate::keys::HostKeys;
use crate::reply::{err, frame};
use crate::request::{self, Made};
use crate::said::Event;
use crate::{records, store};
use alloc::vec;
use alloc::vec::Vec;
use nonos_libc::{mk_ipc_recv_from, mk_ipc_reply, mk_uptime_ms};
use nsh_front_wire::{
    parse, DIGEST_LEN, OP_DOC, OP_ENTRIES, OP_HEALTH, OP_KEYS, OP_RECORDS, OP_SIGN, REQ_MAX,
    ST_BAD_REQUEST, ST_NO_KEYS, ST_OK, ST_REFUSED_CALLER,
};

const OWN_INBOX: u64 = 0;
/// How often the loop wakes while the keys or the request are not done.
const WAKE_MS: u64 = 1_000;
/// 0 waits for a request for ever.
const FOREVER: u64 = 0;

/// The keys and whether the request has been dealt with this boot.
struct Host {
    keys: HostKeys,
    asked: bool,
}

impl Host {
    fn settled(&self) -> bool {
        self.keys.no_tpm() || (self.keys.kept() && self.asked)
    }

    /// Make progress on the keys and the request.
    fn tend(&mut self) {
        let now = uptime();
        if self.keys.get(now).is_err() || self.asked || !self.keys.kept() {
            return;
        }
        let Ok(held) = self.keys.get(now) else { return };
        match request::write(held) {
            Made::Written => {
                crate::say(Event::Request, Some(held.id()));
                self.asked = true;
            }
            Made::Enrolled => self.asked = true,
            Made::NotNow => {}
        }
    }
}

fn uptime() -> u64 {
    u64::try_from(mk_uptime_ms()).unwrap_or(0)
}

pub fn run() -> ! {
    store::bound_calls();
    let mut host = Host { keys: HostKeys::new(), asked: false };
    let mut rx = vec![0u8; REQ_MAX];
    loop {
        host.tend();
        let wait = if host.settled() { FOREVER } else { WAKE_MS };
        let mut sender = 0u32;
        let n = mk_ipc_recv_from(OWN_INBOX, rx.as_mut_ptr(), rx.len(), wait, &mut sender);
        let Some(len) = usize::try_from(n).ok().filter(|&l| l > 0 && l <= rx.len()) else {
            continue;
        };
        if sender == 0 {
            continue;
        }
        let out = handle(&mut host, &rx[..len], sender);
        let _ = mk_ipc_reply(sender, out.as_ptr(), out.len());
    }
}

fn handle(host: &mut Host, req: &[u8], sender: u32) -> Vec<u8> {
    let Some(h) = parse(req) else {
        return err(OP_HEALTH, ST_BAD_REQUEST);
    };
    if !is_front(sender) {
        return err(h.op, ST_REFUSED_CALLER);
    }
    match h.op {
        OP_HEALTH => frame(OP_HEALTH, ST_OK, &[]),
        OP_DOC => document(h.body),
        OP_ENTRIES => entries(),
        OP_KEYS => match host.keys.get(uptime()) {
            Ok(held) => frame(OP_KEYS, ST_OK, &held.public()[..]),
            Err(st) => err(OP_KEYS, st),
        },
        OP_SIGN => sign(host, h.body),
        OP_RECORDS => records::answer(),
        _ => err(h.op, ST_BAD_REQUEST),
    }
}

fn sign(host: &mut Host, body: &[u8]) -> Vec<u8> {
    let Ok(digest) = <&[u8; DIGEST_LEN]>::try_from(body) else {
        return err(OP_SIGN, ST_BAD_REQUEST);
    };
    match host.keys.get(uptime()) {
        Ok(held) => match held.sign(digest) {
            Some(sig) => frame(OP_SIGN, ST_OK, &sig[..]),
            None => err(OP_SIGN, ST_NO_KEYS),
        },
        Err(st) => err(OP_SIGN, st),
    }
}
