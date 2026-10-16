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

//! The zero-knowledge certificate the device proof passes, run on every
//! isolation proof before it leaves: the mask must reach the rank of what FRI
//! reveals past layer zero, or nothing is emitted.

use alloc::format;

use nonos_stark::air::StarkProofExtRounds;
use nonos_stark::field::Fp;
use stark_proofs::zk_rank::check_fri_rank_rounds;

use super::error::Error;
use crate::circuit::shape;
use crate::params::{RANK_ATTEMPTS, STATEMENT_BLOWUP_BITS as EXTRA};
use crate::statement::Statement;

pub(super) fn certify(
    st: &Statement,
    rounds: &StarkProofExtRounds,
    words: &[Fp],
) -> Result<(), Error> {
    let air = shape(st).ok_or(Error::Shape)?;
    let cert =
        check_fri_rank_rounds(air, rounds, words, EXTRA, RANK_ATTEMPTS).map_err(Error::Rank)?;
    if !cert.holds {
        return Err(Error::Rank(format!(
            "{} of {} certified after {} subsets; prove again with fresh entropy",
            cert.mask_rank, cert.bound, cert.attempts
        )));
    }
    Ok(())
}
