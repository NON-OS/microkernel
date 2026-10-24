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

//! The `OP_GET_LINUX_POLICY` reply as init reads it (I-C5): a local copy of
//! nonos_market_proto::linux_policy's wire, as the plan asks of a track that
//! does not link another's crate. market_proofs includes this file and
//! holds it to market_proto's writer (abi_v3/linux_policy_kernel.rs).
//!
//! Body: version 1 | kind (0 none, 1 policy, 2 refused) | for kind 1 only,
//! policy[256] | listing_key[32] | release_key[32]. Anything else is None.

extern crate alloc;

use alloc::boxed::Box;

pub(crate) const VERSION: u8 = 1;
pub(crate) const MARKET_ARG_LEN: usize = 320;

#[derive(Clone, PartialEq, Eq, Debug)]
pub(crate) enum LinuxPolicyReply {
    /// No policy rides with the release: the install role is sent "-".
    NoPolicy,
    /// The bytes of argv m: policy | listing_key | release_key.
    Policy(Box<[u8; MARKET_ARG_LEN]>),
    /// The market does not vouch for the policy: the install is refused.
    Refused,
}

pub(crate) fn parse(body: &[u8]) -> Option<LinuxPolicyReply> {
    match body {
        [VERSION, 0] => Some(LinuxPolicyReply::NoPolicy),
        [VERSION, 2] => Some(LinuxPolicyReply::Refused),
        [VERSION, 1, rest @ ..] => {
            let m: [u8; MARKET_ARG_LEN] = rest.try_into().ok()?;
            Some(LinuxPolicyReply::Policy(Box::new(m)))
        }
        _ => None,
    }
}
