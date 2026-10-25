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

/*
 * The refund's or the reclaim's destination: a Shield
 * address handed out once (mode 1), so no two orders or channels share
 * one; never the pool and never a contract. Its answer is read in
 * fresh_reply.rs.
 */

use shield_wire::{FRESH_ONCE, OP_FRESH_ADDRESS};

use super::fresh_reply::fresh_from;
use crate::wallet::shield::client::call;

pub fn fresh_address() -> Result<[u8; 20], &'static str> {
    let a = call(OP_FRESH_ADDRESS, &[FRESH_ONCE])?;
    fresh_from(a.status, &a.body)
}
