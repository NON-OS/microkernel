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

//! Building a whole three hop circuit.

extern crate alloc;

use alloc::vec::Vec;

use crate::link::Link;
use crate::path::Relay;
use crate::protocol::HOPS;
use crate::trace;

use super::super::hop::Hop;
use super::error::BuildError;
use super::extend::extend;
use super::first::first;

/// Build over `path`, whose first entry must be the relay `link` is to.
///
pub fn build(link: &mut Link, circuit: u32, path: &[Relay]) -> Result<Vec<Hop>, BuildError> {
    if path.len() != HOPS {
        return Err(BuildError::Protocol);
    }
    let mut hops: Vec<Hop> = Vec::with_capacity(HOPS);
    hops.push(first(link, circuit, &path[0])?);
    for next in path.iter().skip(1) {
        let hop = extend(link, circuit, &mut hops, next)?;
        hops.push(hop);
    }
    trace::say_num(b"circuit open hops", hops.len() as u64);
    Ok(hops)
}
