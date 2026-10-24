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
//! init hands it to capsule_linux's install role in argv m (plan/30).
//!
//! Body: version u8 (1) | kind u8 | for KIND_POLICY only, policy[256] |
//! listing_key[32] | release_key[32]. Any other version, kind or length is
//! refused, so a reply is either exactly one of the three or nothing.

use alloc::boxed::Box;
use alloc::vec::Vec;

pub const LINUX_POLICY_VERSION: u8 = 1;
/// No policy rides with this release (a vendor program, whose box is the
/// vendor table's, or no Linux listing): init sends "-".
pub const KIND_NONE: u8 = 0;
/// A policy the market verified under the release's signature and ids.
pub const KIND_POLICY: u8 = 1;
/// A policy the market does not vouch for: the install is refused.
pub const KIND_REFUSED: u8 = 2;
/// policy[256] | listing_key[32] | release_key[32], argv m's bytes.
pub const MARKET_ARG_LEN: usize = 320;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LinuxPolicy {
    NoPolicy,
    Policy(Box<[u8; MARKET_ARG_LEN]>),
    Refused,
}

pub fn parse_linux_policy(body: &[u8]) -> Option<LinuxPolicy> {
    match body {
        [LINUX_POLICY_VERSION, KIND_NONE] => Some(LinuxPolicy::NoPolicy),
        [LINUX_POLICY_VERSION, KIND_REFUSED] => Some(LinuxPolicy::Refused),
        [LINUX_POLICY_VERSION, KIND_POLICY, rest @ ..] => {
            rest.try_into().ok().map(|m: [u8; MARKET_ARG_LEN]| LinuxPolicy::Policy(Box::new(m)))
        }
        _ => None,
    }
}

pub fn linux_policy_body(reply: &LinuxPolicy) -> Vec<u8> {
    let mut out = Vec::with_capacity(MARKET_ARG_LEN.saturating_add(2));
    out.push(LINUX_POLICY_VERSION);
    match reply {
        LinuxPolicy::NoPolicy => out.push(KIND_NONE),
        LinuxPolicy::Refused => out.push(KIND_REFUSED),
        LinuxPolicy::Policy(m) => {
            out.push(KIND_POLICY);
            out.extend_from_slice(m.as_slice());
        }
    }
    out
}
