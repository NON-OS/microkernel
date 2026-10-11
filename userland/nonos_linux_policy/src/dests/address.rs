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

//! An address entry: its length, its prefix and its host bits.

use super::allows::bit;
use super::error::DestsError;

/// ::ffff:0:0/96, where IPv6 spells an IPv4 address.
const MAPPED: [u8; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff];

pub(super) fn v4(body: &[u8], prefix: u8) -> Result<(), DestsError> {
    network(body, 4, prefix)
}

/// An IPv4 network written inside ::ffff:0:0/96 is refused: a list writes
/// it as IPv4, so it has one spelling.
pub(super) fn v6(body: &[u8], prefix: u8) -> Result<(), DestsError> {
    if prefix >= 96 && body.starts_with(&MAPPED) {
        return Err(DestsError::Address);
    }
    network(body, 16, prefix)
}

fn network(body: &[u8], len: usize, prefix: u8) -> Result<(), DestsError> {
    if body.len() != len || usize::from(prefix) > len * 8 {
        return Err(DestsError::Address);
    }
    match (usize::from(prefix)..len * 8).any(|at| bit(body, at)) {
        true => Err(DestsError::HostBits),
        false => Ok(()),
    }
}
