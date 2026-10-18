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

//! _OSC asked for more than is claimed: the query names every capability
//! the operating system could take, so firmware says what it would give
//! up, changing nothing; the commit then claims only what `claim` allows of
//! that. Both answers are kept: what was offered, and what was granted.

use super::eval::Eval;
use super::osc::call;
use super::osc_status::{OscError, QUERY};
use crate::load::Aml;
use crate::namespace::Id;
use alloc::vec::Vec;

/// (offered, granted), each the capability words firmware returned.
pub fn offer_then_claim(
    aml: &mut Aml,
    eval: &mut Eval<'_>,
    osc: Id,
    uuid: (&[u8; 16], u64),
    asked: &[u32],
    claim: &[u32],
) -> Result<(Vec<u32>, Vec<u32>), OscError> {
    let mut query = asked.to_vec();
    query[0] = QUERY;
    let offered = call(aml, eval, osc, uuid, &query)?;
    let mask = |i: usize| claim.get(i).copied().unwrap_or(0);
    let mut commit: Vec<u32> =
        asked.iter().zip(&offered).enumerate().map(|(i, (a, o))| a & o & mask(i)).collect();
    commit[0] = 0;
    let granted = call(aml, eval, osc, uuid, &commit)?;
    Ok((offered, granted))
}
