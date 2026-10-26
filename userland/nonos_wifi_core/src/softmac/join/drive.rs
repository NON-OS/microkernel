// NONOS Operating System (AGPL-3.0-or-later)

//! Run the association state machine against the radio until it connects,
//! fails, or the budget runs out, resending what the air lost (`send`).

use alloc::vec::Vec;

use super::report::{Outcome, Report};
use super::report_of::report;
use super::send::{resend_after, transmit};
use super::{announce::announce, counters::Counters, feed::feed, finish::finish};
use crate::mlme::{Mlme, MlmeState};
use crate::rsn::Cipher;
use crate::softmac::radio::{Env, Radio};

pub fn drive<R: Radio, E: Env>(
    r: &mut R,
    env: &E,
    mut m: Mlme,
    beacon: &[u8],
    budget_ms: u64,
    seq_start: u16,
) -> Report {
    let (mut c, mut buf) = (Counters::default(), [0u8; 2400]);
    let (start, mut seq) = (env.now_ms(), seq_start & 0x0FFF); // random start (E-70)
    let mut quiet_since = start;
    let mut pending: Vec<u8> = m.on_mgmt(beacon).tx.unwrap_or_default();
    if !pending.is_empty() {
        transmit(r, &mut pending, &mut seq, &mut c);
    }
    loop {
        let now = env.now_ms();
        m.expire(now);
        match m.state() {
            MlmeState::Connected => return report(&m, finish(&m), c),
            MlmeState::Failed if m.expired() => return report(&m, Outcome::TimedOut, c),
            MlmeState::Failed => return report(&m, Outcome::Refused, c),
            _ => {}
        }
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
        let cipher = m.selection().map_or(Cipher::Ccmp128, |s| s.pairwise);
        announce(r, (before, m.state()), frame, (beacon, cipher));
    }
}
