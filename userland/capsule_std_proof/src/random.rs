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

//! Randomness as std draws it: RandomState seeds each map's hasher from the
//! platform's source, which on NONOS is the kernel's generator. Two states
//! hashing the same value must disagree.

use std::collections::hash_map::RandomState;
use std::hash::BuildHasher;

pub fn prove() -> Result<String, String> {
    let draws: Vec<u64> = (0..4).map(|_| RandomState::new().hash_one(0x4e4f_4e4f_u64)).collect();
    let distinct = draws.iter().enumerate().all(|(i, a)| draws[..i].iter().all(|b| a != b));
    match distinct {
        true => Ok(format!("{} RandomState seeds, all distinct", draws.len())),
        false => Err(format!("seeds repeated: {draws:x?}")),
    }
}
