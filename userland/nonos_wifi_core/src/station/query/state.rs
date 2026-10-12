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

//! What an SA Query holds while it runs.

/// dot11AssociationSAQueryRetryTimeout.
pub const RETRY_MS: u64 = 201;
/// dot11AssociationSAQueryMaximumTimeout.
pub const MAX_MS: u64 = 1_000;

#[derive(Clone, Copy)]
pub(super) struct Asking {
    /// When the first request went; stamped at the next tick if no clock ran.
    pub(super) started: Option<u64>,
    pub(super) next_ms: u64,
    pub(super) first: u16,
    pub(super) sent: u16,
    pub(super) reason: u16,
}

#[derive(Clone, Copy)]
pub(in crate::station) struct Query {
    pub(super) asking: Option<Asking>,
    pub(super) next_id: u16,
}

impl Query {
    pub(in crate::station) const fn new() -> Self {
        Self { asking: None, next_id: 1 }
    }

    /// Start asking about a deauthentication with `reason`: the identifier of
    /// the first request, or `None` while a query already runs.
    pub(in crate::station) fn start(&mut self, now: Option<u64>, reason: u16) -> Option<u16> {
        if self.asking.is_some() {
            return None;
        }
        let first = self.take_id();
        let next_ms = now.map_or(0, |n| n.saturating_add(RETRY_MS));
        self.asking = Some(Asking { started: now, next_ms, first, sent: 1, reason });
        Some(first)
    }

    /// A response with `id`: whether it answers this query, which then ends.
    pub(in crate::station) fn answered(&mut self, id: u16) -> bool {
        let ours = self.asking.is_some_and(|a| id.wrapping_sub(a.first) < a.sent);
        if ours {
            self.asking = None;
        }
        ours
    }
}
