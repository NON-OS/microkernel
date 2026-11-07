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

//! The proofs' keys are made from their fixed seeds: no space is set, and
//! once a key is made none can be.

use crate::build::keys::{key, reseed, space, K};

#[test]
fn a_space_is_refused_once_a_key_is_made() {
    let _ = key(K::User);
    assert!(!reseed([0x5A; 32]));
    assert!(space().is_empty(), "the proofs' keys are the fixed ones");
}
