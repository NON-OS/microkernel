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

//! Writing OP_GET_LINUX_POLICY's reply, as the market sends it.

use alloc::vec::Vec;

use super::{
    LinuxPolicy, KIND_NONE, KIND_POLICY, KIND_REFUSED, LINUX_POLICY_HEAD, LINUX_POLICY_VERSION,
    MARKET_ARG_LEN,
};

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
