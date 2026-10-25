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

//! A kernel return as the job's failure status: the errno when it is one,
//! PROVER for anything else, so a failed job never reports 0 or a value cut
//! down from a wider one.

use nonos_proof_abi::wire::PROVER;

pub fn failed(n: i64) -> i32 {
    i32::try_from(n).ok().filter(|&s| s < 0).unwrap_or(PROVER)
}
