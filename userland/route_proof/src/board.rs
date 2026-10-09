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

//! The board the attest service keeps: the latest report from each transport,
//! and when it arrived.
//!
//! A post is accepted only from the transport for the report's own network,
//! named and capable as the kernel's process table says (`may_report`), and
//! only if it decodes exactly. A refused post changes nothing. The answer gives
//! each report with its age, measured on the board's clock at the moment of
//! asking, so a reader judges freshness without trusting the sender's clock.

use crate::authorize::may_report;
use crate::report::{decode, encode, Network, RouteReport, REPORT_LEN};

/// One slot on the wire: present (0 or 1), age in milliseconds, the report.
pub const SLOT_LEN: usize = 1 + 8 + REPORT_LEN;
/// The answer: the Nym slot, then the Anyone slot.
pub const ANSWER_LEN: usize = 2 * SLOT_LEN;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PostError {
    /// Not exactly one well formed report.
    Malformed,
    /// The sender is not the transport for that network, or holds no Network.
    NotPermitted,
}

#[derive(Clone, Copy, Default)]
pub struct Board {
    nym: Option<(RouteReport, u64)>,
    anyone: Option<(RouteReport, u64)>,
}

impl Board {
    pub const fn new() -> Self {
        Self { nym: None, anyone: None }
    }

    /// Take `payload` from the process the kernel names `sender_name`, holding
    /// `sender_caps`, at board time `now_ms`.
    pub fn post(
        &mut self,
        sender_name: &[u8],
        sender_caps: u64,
        payload: &[u8],
        now_ms: u64,
    ) -> Result<Network, PostError> {
        let report = decode(payload).ok_or(PostError::Malformed)?;
        if !may_report(sender_name, sender_caps, report.network) {
            return Err(PostError::NotPermitted);
        }
        let slot = match report.network {
            Network::Nym => &mut self.nym,
            Network::Anyone => &mut self.anyone,
        };
        *slot = Some((report, now_ms));
        Ok(report.network)
    }

    /// Forget a transport's report, as when its process is gone.
    pub fn clear(&mut self, network: Network) {
        match network {
            Network::Nym => self.nym = None,
            Network::Anyone => self.anyone = None,
        }
    }

    /// The latest report for `network` and its age at `now_ms`.
    pub fn latest(&self, network: Network, now_ms: u64) -> Option<(RouteReport, u64)> {
        let slot = match network {
            Network::Nym => self.nym,
            Network::Anyone => self.anyone,
        };
        slot.map(|(r, at)| (r, now_ms.saturating_sub(at)))
    }

    pub fn answer(&self, now_ms: u64) -> [u8; ANSWER_LEN] {
        let mut out = [0u8; ANSWER_LEN];
        put(&mut out[..SLOT_LEN], self.latest(Network::Nym, now_ms));
        put(&mut out[SLOT_LEN..], self.latest(Network::Anyone, now_ms));
        out
    }
}

fn put(slot: &mut [u8], latest: Option<(RouteReport, u64)>) {
    if let Some((r, age)) = latest {
        slot[0] = 1;
        slot[1..9].copy_from_slice(&age.to_le_bytes());
        slot[9..].copy_from_slice(&encode(&r));
    }
}

/// A board answer as a reader sees it: (Nym, Anyone), each with its age.
/// `None` for an answer that is not exactly one, or a slot that claims a
/// report of the other network.
pub type Latest = Option<(RouteReport, u64)>;

pub fn read_answer(b: &[u8]) -> Option<(Latest, Latest)> {
    if b.len() != ANSWER_LEN {
        return None;
    }
    let nym = slot(&b[..SLOT_LEN], Network::Nym)?;
    let anyone = slot(&b[SLOT_LEN..], Network::Anyone)?;
    Some((nym, anyone))
}

fn slot(b: &[u8], network: Network) -> Option<Latest> {
    match b[0] {
        0 => {
            if b[1..].iter().any(|&x| x != 0) {
                return None;
            }
            Some(None)
        }
        1 => {
            let age = u64::from_le_bytes([b[1], b[2], b[3], b[4], b[5], b[6], b[7], b[8]]);
            let r = decode(&b[9..])?;
            if r.network != network {
                return None;
            }
            Some(Some((r, age)))
        }
        _ => None,
    }
}
