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

//! Deciding a grant for the live family of `pid`, bound to that family by its
//! generation: read before the facts and again after, so a family that ended
//! while they were read is no family's grant.

use super::{by_box, by_policy, Facts, Grant, Refusal};

/// What a family asks to be granted on.
pub enum Ask<'a> {
    /// Its program's policy, these bytes.
    Policy(&'a [u8]),
    /// Its run's box.
    Box,
}

pub fn decide(pid: u32, ask: Ask<'_>, facts: &impl Facts) -> Result<(Grant, u64), Refusal> {
    let generation = facts.generation(pid);
    if generation == 0 {
        return Err(Refusal::NoFamily);
    }
    let grant = match ask {
        Ask::Policy(policy) => by_policy(pid, policy, facts)?,
        Ask::Box => by_box(pid, facts)?,
    };
    match facts.generation(pid) == generation {
        true => Ok((grant, generation)),
        false => Err(Refusal::NoFamily),
    }
}
