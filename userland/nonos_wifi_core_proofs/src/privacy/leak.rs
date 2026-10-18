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

//! Whether a secret ever left the station: in any frame it sent on the
//! air, or in any reply it gave its callers.

use crate::soft::rig::Station;
use crate::soft::wire::control;

fn holds(haystack: &[u8], needle: &[u8]) -> bool {
    haystack.windows(needle.len()).any(|w| w == needle)
}

/// Every frame sent, and the status, link and scan replies.
pub fn leaked(st: &mut Station, secret: &[u8]) -> bool {
    let replies = [4u16, 5, 3].map(|op| control(st, op, &[]));
    let sent = &st.radio().unwrap().sent;
    sent.iter().chain(replies.iter()).any(|f| holds(f, secret))
}
