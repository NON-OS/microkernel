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

//! Run the association state machine against the radio until it connects,
//! fails, or the budget runs out, resending what the air lost (`send`).

use alloc::vec::Vec;

use super::counters::Counters;
use super::feed::feed;
use super::report::{finish, report, Outcome, Report};
use super::send::{resend_after, transmit};
use crate::mlme::{Mlme, MlmeState};
use crate::softmac::assoc::announce;
use crate::softmac::radio::{Env, Radio};

pub fn drive<R: Radio, E: Env>(
    r: &mut R,
    env: &E,
    mut m: Mlme,
    beacon: &[u8],
    budget_ms: u64,
) -> Report {
    let mut c = Counters::default();
    let (start, mut seq) = (env.now_ms(), 0u16);
    let mut quiet_since = start;
    let mut pending: Vec<u8> = m.on_mgmt(beacon).tx.unwrap_or_default();
    if !pending.is_empty() {
        transmit(r, &mut pending, &mut seq, &mut c);
    }
    let mut buf = [0u8; 2400];
    loop {
        match m.state() {
            MlmeState::Connected => return report(&m, finish(&m), c),
            MlmeState::Failed => return report(&m, Outcome::Refused, c),
            _ => {}
        }
        let now = env.now_ms();
        if now.saturating_sub(start) >= budget_ms {
            return report(&m, Outcome::TimedOut, c);
        }
        if !pending.is_empty() && now.saturating_sub(quiet_since) >= resend_after(&pending) {
            transmit(r, &mut pending, &mut seq, &mut c);
            quiet_since = now;
        }
        let Some(rx) = r.recv(&mut buf) else { continue };
        c.recv += 1;
        let (before, frame) = (m.state(), &buf[..rx.len.min(buf.len())]);
        match feed(&mut m, frame, &mut c) {
            Some(mut f) => {
                transmit(r, &mut f, &mut seq, &mut c);
                (pending, quiet_since) = (f, now);
            }
            // The peer speaks next (the AP opens the four-way handshake).
            None if m.state() != before => (pending, quiet_since) = (Vec::new(), now),
            None => {}
        }
        announce(r, (before, m.state()), frame, beacon);
    }
}
