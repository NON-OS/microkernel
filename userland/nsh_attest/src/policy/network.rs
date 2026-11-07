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

//! A release's network stack, held to its rules before the policy is used.

use super::MAX_NETWORK;
use super::{PolicyError, Release};

/// A release's network set: at most `MAX_NETWORK` measurements, each once,
/// none of them the front.
pub(super) fn network(r: &Release) -> Result<(), PolicyError> {
    let n = &r.network;
    let twice = |i: usize| n[..i].contains(&n[i]);
    match n.len() > MAX_NETWORK || n.contains(&r.front) || (0..n.len()).any(twice) {
        true => Err(PolicyError::Network),
        false => Ok(()),
    }
}
