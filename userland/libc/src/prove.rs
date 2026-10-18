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

//! The store's ask for the headless prover.

use crate::syscall::{call_raw, N_MK_PROVE_JOB};

/// PROVE, the store's: the person's sealed run, proven for a listing.
pub const PROVE_JOB_PROVE: u64 = 1;
/// DELIVER, a vendor supervisor's: one served request, attested.
pub const PROVE_JOB_DELIVER: u64 = 2;

/// Ask init for nonos.prove on app.prove.job, bound to this caller: 0, or
/// EBUSY while one is asked for, a prover runs, or the DELIVER rate holds,
/// or EPERM for a caller the kind does not take. The frame itself goes to
/// app.prove.job in nonos_proof_abi::wire's frames.
pub fn mk_prove_job(kind: u64) -> i64 {
    call_raw(N_MK_PROVE_JOB, [kind, 0, 0, 0, 0, 0])
}
