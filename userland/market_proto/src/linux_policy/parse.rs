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

//! Reading OP_GET_LINUX_POLICY's reply: exactly one of the three kinds, at
//! exactly its length, or nothing.

use alloc::boxed::Box;

use super::{LinuxPolicy, KIND_NONE, KIND_POLICY, KIND_REFUSED, LINUX_POLICY_VERSION};

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
