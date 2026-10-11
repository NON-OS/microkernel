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

//! plan/13 step 5 as the prover holds it before proving anything: the record
//! names this policy's app, and a market policy names this listing. A vendor
//! policy (ids zero) is held to the vendor table at I-A1.

use nonos_proof_abi::wire::POLICY;
use nonos_proof_abi::{app_id, Record};

/// The policy's program: plan/30's program_blake3 at offset 16.
pub fn program(policy: &[u8; 256]) -> [u8; 32] {
    let mut p = [0u8; 32];
    p.copy_from_slice(&policy[16..48]);
    p
}

pub fn check(policy: &[u8; 256], listing_id: &[u8; 32], rec: &Record) -> Result<(), i32> {
    if rec.app_id != app_id(policy, &program(policy)) {
        return Err(POLICY);
    }
    let named = &policy[56..88];
    if named != [0u8; 32] && named != listing_id {
        return Err(POLICY);
    }
    Ok(())
}
