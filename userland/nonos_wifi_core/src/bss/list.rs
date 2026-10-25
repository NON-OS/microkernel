// NONOS Operating System (AGPL-3.0-or-later)

//! The candidates a hunt heard for one network. A frame this station could
//! not join from (open, a malformed RSNE, a downgrade under the policy) is
//! no candidate, only the reason given if none is found. The join takes the
//! best candidate and, if that join fails, the next, at most `MAX_TRIES`
//! times: no single unauthenticated beacon decides the join or ends it.

use alloc::vec::Vec;

use super::candidate::{rank, Candidate};
use crate::mlme::{judge, MlmeFailure};
use crate::rsn::JoinPolicy;

/// The most candidates kept; past it a better one replaces the worst.
pub const MAX_CANDIDATES: usize = 8;
/// The most joins one connect request makes.
pub const MAX_TRIES: usize = 3;
/// The longest frame kept (the 802.11 MSDU limit).
pub const FRAME_MAX: usize = 2304;

pub struct Candidates {
    ssid: Vec<u8>,
    policy: JoinPolicy,
    pub(super) list: Vec<Candidate>,
    pub(super) refusal: Option<MlmeFailure>,
    pub(super) tried: usize,
}

impl Candidates {
    pub fn new(ssid: &[u8], policy: JoinPolicy) -> Self {
        let ssid = ssid.to_vec();
        Self { ssid, policy, list: Vec::new(), refusal: None, tried: 0 }
    }

    /// Hear `frame` on join channel `channel` at `signal` (0 to 100).
    pub fn hear(&mut self, frame: &[u8], channel: u8, signal: u8) {
        if frame.len() > FRAME_MAX {
            return;
        }
        let j = match judge(frame, &self.ssid, self.policy) {
            None => return,
            Some(Err(failure)) => return self.refusal = Some(failure),
            Some(Ok(j)) => j,
        };
        let ies = [j.rsne, j.rsnxe.unwrap_or(&[])].concat();
        let bssid = j.info.bssid;
        if let Some(c) = self.list.iter_mut().find(|c| c.is(bssid, channel, &ies)) {
            c.frame = frame.to_vec();
            c.signal = signal;
            c.heard = c.heard.saturating_add(1);
            return;
        }
        let rank = rank(j.selection.akm);
        let new = Candidate { frame: frame.to_vec(), channel, bssid, ies, rank, signal, heard: 1 };
        if self.list.len() < MAX_CANDIDATES {
            return self.list.push(new);
        }
        let worst = (0..self.list.len()).min_by_key(|&i| self.list[i].key());
        if let Some(i) = worst.filter(|&i| self.list[i].key() < new.key()) {
            self.list[i] = new;
        }
    }
}
