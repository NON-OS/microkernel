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

//! The station's SA Query procedure (IEEE Std 802.11-2020, 11.13): an
//! unprotected deauthentication may be a forgery, or the AP may really have
//! lost the association. The station asks, protected; a protected answer with
//! one of the transaction identifiers it sent keeps the link, and silence for
//! the maximum timeout ends it. Each retry carries a fresh identifier.

/// dot11AssociationSAQueryRetryTimeout, 201 TU, in milliseconds.
pub const SA_QUERY_RETRY_MS: u64 = 206;
/// dot11AssociationSAQueryMaximumTimeout, 1000 TU, in milliseconds.
pub const SA_QUERY_MAX_MS: u64 = 1024;
const MAX_TXIDS: usize = 8;

/// What a tick of the procedure calls for.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SaPoll {
    /// Nothing to send yet.
    Wait,
    /// Send another request with this identifier.
    Retry([u8; 2]),
    /// No answer in time: the association is gone.
    Expired,
}

#[derive(Clone, Copy, Debug)]
pub struct SaQuery {
    started_ms: u64,
    sent_ms: u64,
    txids: [[u8; 2]; MAX_TXIDS],
    sent: usize,
}

impl SaQuery {
    /// Start the procedure with the first request's identifier.
    pub fn start(now_ms: u64, txid: [u8; 2]) -> Self {
        let mut txids = [[0u8; 2]; MAX_TXIDS];
        txids[0] = txid;
        Self { started_ms: now_ms, sent_ms: now_ms, txids, sent: 1 }
    }

    /// Advance to `now_ms`; `fresh` is the identifier a retry would use.
    pub fn poll(&mut self, now_ms: u64, fresh: [u8; 2]) -> SaPoll {
        if now_ms.saturating_sub(self.started_ms) >= SA_QUERY_MAX_MS {
            return SaPoll::Expired;
        }
        if now_ms.saturating_sub(self.sent_ms) < SA_QUERY_RETRY_MS || self.sent == MAX_TXIDS {
            return SaPoll::Wait;
        }
        self.txids[self.sent] = fresh;
        self.sent += 1;
        self.sent_ms = now_ms;
        SaPoll::Retry(fresh)
    }

    /// Whether a response's identifier is one the station sent.
    pub fn answers(&self, txid: [u8; 2]) -> bool {
        self.txids[..self.sent].contains(&txid)
    }
}
