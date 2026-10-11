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

extern crate alloc;

use alloc::vec::Vec;

use super::error::DecodeError;
use super::reader::Reader;
use super::strings::bounded_count;
use crate::limits::MAX_PROOF_REQS;
use crate::types::{Commerce, ProofReq, SettleMode};

/// The plan/11 container kinds: isolation, delivery, authenticity.
pub(super) fn known_proof_kind(kind: u8) -> bool {
    matches!(kind, 1..=3)
}

pub(super) fn read(r: &mut Reader<'_>) -> Result<Commerce, DecodeError> {
    let payee = r.fixed::<20>()?;
    let bond_id = r.fixed::<32>()?;
    let unit = r.fixed::<32>()?;
    let unit_price = r.u128()?;
    let min_charge = r.u128()?;
    let settle_mode = SettleMode::from_u8(r.u8()?).ok_or(DecodeError::UnknownKind)?;

    let n = bounded_count(r, MAX_PROOF_REQS)?;
    let mut proof_reqs = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let kind = r.u8()?;
        if !known_proof_kind(kind) {
            return Err(DecodeError::UnknownKind);
        }
        proof_reqs.push(ProofReq { kind, statement: r.fixed::<32>()? });
    }

    let binding_sig = r.fixed::<65>()?;
    Ok(Commerce {
        payee,
        bond_id,
        unit,
        unit_price,
        min_charge,
        settle_mode,
        proof_reqs,
        binding_sig,
    })
}
