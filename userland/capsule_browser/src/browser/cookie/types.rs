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

//! One stored cookie, as RFC 6265 section 5.3 keeps it.

use alloc::string::String;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    /// Lowercase, without a leading dot.
    pub domain: String,
    /// Set without a Domain attribute: sent to exactly this host and no
    /// subdomain of it.
    pub host_only: bool,
    pub path: String,
    /// Sent only over https.
    pub secure: bool,
    /// Never shown to page scripts.
    pub http_only: bool,
    /// Unix seconds after which the cookie is gone; `None` lasts as long as
    /// the jar does, which is until the browser exits.
    pub expires: Option<i64>,
    /// Creation order, kept across a replacement, for ordering and eviction.
    pub seq: u64,
}

impl Cookie {
    pub fn expired(&self, now: i64) -> bool {
        self.expires.is_some_and(|at| at <= now)
    }
}
