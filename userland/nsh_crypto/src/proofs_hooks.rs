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

//! For the proof crate only, behind the `proofs` feature: a sealer or an
//! opener whose count starts where the proof says, so the 2^32 limits can
//! be reached. A real session always starts at zero.

use crate::count::Count;
use crate::{Opener, Sealer, Secret};

impl Sealer {
    pub fn starting_at(key: Secret<32>, seq: u64, bytes: u64) -> Self {
        Sealer::from_count(Count { key, seq, bytes })
    }
}

impl Opener {
    pub fn starting_at(key: Secret<32>, seq: u64, bytes: u64) -> Self {
        Opener::from_count(Count { key, seq, bytes })
    }
}
