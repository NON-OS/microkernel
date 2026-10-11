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

//! The quota the store can carry, and a change past either ceiling.

use crate::calls::loadavg::declared::{MEMORY, PRIVATE, PRIVATE_NAMES};
use crate::calls::loadavg::space::quota::{allows, Kept};
use crate::linux::abi::errno::ENOSPC;

/// capsule_vfs: the names the whole store holds, and the heap its bytes
/// live in beside the 60 MiB it loads at boot.
pub(super) const STORE_NAMES: u64 = 2048;
const STORE_HEAP: u64 = 192 << 20;
const STORE_LOADED: u64 = 60 << 20;

pub(super) fn kept(bytes: u64, names: u64) -> Kept {
    Kept { bytes, names }
}

/// Checked as the crate compiles: a ceiling raised past what the store
/// carries fails the build of these proofs.
#[test]
fn the_quota_is_one_the_store_can_carry_for_several_families() {
    const { assert!(PRIVATE < MEMORY / 2, "not the half of memory an unsized tmpfs takes") };
    const { assert!(8 * PRIVATE <= STORE_HEAP - STORE_LOADED, "eight families fit the heap") };
    const { assert!(8 * PRIVATE_NAMES <= STORE_NAMES / 2, "eight take half the names") };
    const { assert!(PRIVATE >= 8 << 20, "one file as large as a family may hold fits") };
}

#[test]
fn a_family_within_its_quota_is_never_refused() {
    assert_eq!(allows(kept(0, 0), kept(PRIVATE, PRIVATE_NAMES)), Ok(()));
    assert_eq!(allows(kept(PRIVATE - 1, 0), kept(PRIVATE, 1)), Ok(()));
}

#[test]
fn a_change_past_either_ceiling_is_enospc() {
    assert_eq!(allows(kept(PRIVATE, 0), kept(PRIVATE + 1, 0)), Err(ENOSPC));
    assert_eq!(allows(kept(0, PRIVATE_NAMES), kept(0, PRIVATE_NAMES + 1)), Err(ENOSPC));
    assert_eq!(allows(kept(0, 0), kept(u64::MAX, 0)), Err(ENOSPC));
    let full = kept(PRIVATE, PRIVATE_NAMES);
    assert_eq!(allows(full, full.plus(kept(0, 1))), Err(ENOSPC), "one more name");
    assert_eq!(allows(full, full.plus(kept(1, 0))), Err(ENOSPC), "one more byte");
}
