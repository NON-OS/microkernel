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

//! `OP_GET_LINUX_POLICY` (I-C5): the box a Linux release is sold with, as
//! init hands it to capsule_linux's install role in argv m.
//!
//! Body: version u8 (2) | kind u8 | package_hash[32] | for KIND_POLICY
//! only, policy[256] | listing_key[32] | release_key[32]. The package hash is
//! the release the answer was made from, which init holds to the pin it
//! was given, so an index replaced between its asks is refused, never mixed.
//! Any other version, kind or length is refused, so a reply is exactly one
//! of the three or nothing.

use alloc::boxed::Box;
use alloc::vec::Vec;

pub const LINUX_POLICY_VERSION: u8 = 2;
/// No policy rides with this release (a vendor program, whose box is the
/// vendor table's, or no Linux listing): init sends "-".
pub const KIND_NONE: u8 = 0;
/// A policy the market verified under the release's signature and ids.
pub const KIND_POLICY: u8 = 1;
/// A policy the market does not vouch for: the install is refused.
pub const KIND_REFUSED: u8 = 2;
/// policy[256] | listing_key[32] | release_key[32], argv m's bytes.
pub const MARKET_ARG_LEN: usize = 320;
/// version | kind | package_hash.
pub const LINUX_POLICY_HEAD: usize = 34;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LinuxPolicy {
    NoPolicy,
    Policy(Box<[u8; MARKET_ARG_LEN]>),
    Refused,
}

/// The answer and the package hash of the release it was made from.
pub fn parse_linux_policy(body: &[u8]) -> Option<(LinuxPolicy, [u8; 32])> {
    let [LINUX_POLICY_VERSION, kind, rest @ ..] = body else { return None };
    let (hash, rest) = rest.split_first_chunk::<32>()?;
    let reply = match (*kind, rest) {
        (KIND_NONE, []) => LinuxPolicy::NoPolicy,
        (KIND_REFUSED, []) => LinuxPolicy::Refused,
        (KIND_POLICY, m) => LinuxPolicy::Policy(Box::new(m.try_into().ok()?)),
        _ => return None,
    };
    Some((reply, *hash))
}

pub fn linux_policy_body(reply: &LinuxPolicy, package_hash: &[u8; 32]) -> Vec<u8> {
    let mut out = Vec::with_capacity(LINUX_POLICY_HEAD.saturating_add(MARKET_ARG_LEN));
    out.push(LINUX_POLICY_VERSION);
    out.push(match reply {
        LinuxPolicy::NoPolicy => KIND_NONE,
        LinuxPolicy::Refused => KIND_REFUSED,
        LinuxPolicy::Policy(_) => KIND_POLICY,
    });
    out.extend_from_slice(package_hash);
    if let LinuxPolicy::Policy(m) = reply {
        out.extend_from_slice(m.as_slice());
    }
    out
}
