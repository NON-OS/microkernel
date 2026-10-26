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

//! The box a Linux release is sold with (op 8, I-C5), as init asks for it:
//! policy[256] | listing_key[32] | release_key[32], for a proof of its run.

use alloc::boxed::Box;

use nonos_market_proto::OP_GET_LINUX_POLICY;
use nonos_market_proto::{pair_body, parse_linux_policy, LinuxPolicy, MARKET_ARG_LEN};

use super::wire::exchange;

/// The box of `release` of `listing`, an empty release for the default.
/// None for a release sold with none, one the market does not vouch for,
/// or no answer that reads.
pub fn fetch(
    port: u32,
    request_id: u32,
    listing: &[u8],
    release: &[u8],
) -> Option<Box<[u8; MARKET_ARG_LEN]>> {
    let body = pair_body(listing, release)?;
    let reply = exchange(port, OP_GET_LINUX_POLICY, request_id, &body).ok()?;
    match parse_linux_policy(&reply)?.0 {
        LinuxPolicy::Policy(m) => Some(m),
        LinuxPolicy::NoPolicy | LinuxPolicy::Refused => None,
    }
}
