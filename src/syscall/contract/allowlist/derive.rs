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

use crate::capabilities::CapabilityToken;
use crate::syscall::abi::index::{entry, COUNT};

use super::super::cap_table::is_allowed;
use super::set::Allowlist;

/*
 * Every slot asked of the capability tables twice, once per launcher
 * answer, with the token as the resolver sees it. Expiry is the one input
 * the tables read that changes with time; the resolver refuses an expired
 * token before it consults the list, so the list is derived as of a token
 * that has not expired. Process tokens carry no expiry at all
 * (`process::caps::new_token`).
 */
pub(crate) fn derive(token: &CapabilityToken) -> Allowlist {
    let mut live = token.clone();
    live.expires_at_ms = None;
    let mut list = Allowlist::EMPTY;
    for slot in 0..COUNT {
        if let Some(e) = entry(slot) {
            list.set(slot, is_allowed(&live, e.variant, false), is_allowed(&live, e.variant, true));
        }
    }
    list
}

/*
 * The list a process holds for `token`. The token's MAC is checked here,
 * once, when the token is installed, and a token that fails it permits
 * nothing. With `nonos-syscall-token-mac-every-call` the resolver checks it
 * on every call as well.
 */
pub(crate) fn for_token(token: &CapabilityToken) -> Allowlist {
    if !crate::capabilities::token::verify_token(token) {
        return Allowlist::EMPTY;
    }
    derive(token)
}
