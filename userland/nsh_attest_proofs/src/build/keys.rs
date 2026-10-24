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

//! The hybrid keys the proofs use, made once: ML-DSA-65 key generation is
//! the slow part of every fixture.

use std::sync::OnceLock;

use super::hybrid::Hybrid;

/// The host's keys.
pub const HOST: u8 = 2;
/// The fleet's enrolment authority.
pub const AUTHORITY: u8 = 1;
/// An authority the policy does not hold.
pub const STRANGER: u8 = 3;

static KEYS: [OnceLock<Hybrid>; 4] =
    [OnceLock::new(), OnceLock::new(), OnceLock::new(), OnceLock::new()];

pub fn hybrid(tag: u8) -> &'static Hybrid {
    KEYS[tag as usize].get_or_init(|| Hybrid::new(tag))
}
