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

//! prlimit64 takes a new limit only when it is the limit already there
//! (call/limits/same.rs): gpg's "core dumps off" is that, and is taken; a
//! raise or a lowering is not, since nothing would keep it.

use crate::calls::rlimit_same::same;

fn rlimit(soft: u64, hard: u64) -> [u8; 16] {
    let mut b = [0u8; 16];
    b[..8].copy_from_slice(&soft.to_le_bytes());
    b[8..].copy_from_slice(&hard.to_le_bytes());
    b
}

#[test]
fn the_limit_as_it_stands_is_taken() {
    assert!(same((0, 0), &rlimit(0, 0)), "RLIMIT_CORE set to 0, as gpg does");
    assert!(same((u64::MAX, u64::MAX), &rlimit(u64::MAX, u64::MAX)));
}

#[test]
fn any_other_limit_is_not() {
    assert!(!same((0, 0), &rlimit(0, 1)), "a raised hard limit");
    assert!(!same((1024, 1024), &rlimit(512, 1024)), "a lowered soft limit, not kept");
    assert!(!same((1024, 1024), &rlimit(1024, 512)), "the halves are not swapped");
    assert!(!same((0, 0), &rlimit(0, 0)[..15]), "a short struct");
}
