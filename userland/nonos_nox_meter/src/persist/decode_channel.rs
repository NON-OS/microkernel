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

//! One channel slot of the state file, as layout.rs lays it out.

use super::cursor::Cursor;
use crate::channel::Channel;

pub(super) fn channel(r: &mut Cursor<'_>) -> Option<Channel> {
    Some(Channel {
        id: r.fixed()?,
        payee: r.fixed()?,
        listing_id: r.fixed()?,
        chain_id: r.u64()?,
        open_nonce: r.u64()?,
        unit: r.fixed()?,
        deposit: r.u128()?,
        settled: r.u128()?,
        cumulative: r.u128()?,
        seq: r.u64()?,
        expiry: r.u64()?,
        usage_root: r.fixed()?,
        sig: r.fixed()?,
        frozen: match r.fixed::<1>()?[0] {
            0 => false,
            1 => true,
            _ => return None,
        },
        carried: r.u128()?,
        reclaim_to: r.fixed()?,
    })
}
