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

//! INSTALL_READY's body: the listing and release, each length-prefixed, then
//! the chain_id (u64 LE) the bond gate reads on (v1.4.23). The old form,
//! without it, reads as no chain.

use crate::server::lp::take_lp;

pub(super) fn parse_ready(buf: &[u8]) -> Option<(&str, &str, Option<u64>)> {
    let (first, rest) = take_lp(buf)?;
    let (second, rest) = take_lp(rest)?;
    let chain = match rest.len() {
        0 => None,
        _ => Some(u64::from_le_bytes(rest.try_into().ok()?)),
    };
    Some((core::str::from_utf8(first).ok()?, core::str::from_utf8(second).ok()?, chain))
}
