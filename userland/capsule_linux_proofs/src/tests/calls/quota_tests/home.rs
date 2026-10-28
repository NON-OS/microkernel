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

//! A home's own quota (declared::HOME, HOME_NAMES, or more for a listing
//! home_listings.rs names) and every home's together (declared::HOMES): past
//! any is ENOSPC, a shrink always passes, and the ceilings are ones the
//! store carries beside every family's scratch.

use crate::calls::loadavg::declared::{HOME, HOMES, HOME_MOST, HOME_NAMES, PRIVATE};
use crate::calls::loadavg::space::quota::{allows, home_allows, Kept};
use crate::linux::abi::errno::ENOSPC;

fn k(bytes: u64, names: u64) -> Kept {
    Kept { bytes, names }
}

#[test]
fn a_home_grows_to_its_ceiling_and_no_further() {
    assert_eq!(home_allows(k(0, 0), k(HOME, HOME_NAMES), (0, HOME), HOME), Ok(()));
    assert_eq!(home_allows(k(HOME, 1), k(HOME + 1, 1), (HOME, HOME + 1), HOME), Err(ENOSPC));
    assert_eq!(home_allows(k(0, HOME_NAMES), k(0, HOME_NAMES + 1), (0, 0), HOME), Err(ENOSPC));
}

#[test]
fn every_home_together_is_held_to_homes() {
    let all = (HOMES - 10, HOMES + 10);
    assert_eq!(home_allows(k(0, 1), k(20, 1), all, HOME), Err(ENOSPC), "one app under its own");
    assert_eq!(home_allows(k(0, 1), k(10, 1), (HOMES - 10, HOMES), HOME), Ok(()));
}

#[test]
fn a_full_home_still_shrinks_and_removes() {
    let over = k(HOME + 5, HOME_NAMES + 2);
    assert_eq!(
        home_allows(over, k(HOME + 1, HOME_NAMES + 1), (HOMES + 5, HOMES + 1), HOME),
        Ok(())
    );
}

#[test]
fn the_home_and_the_scratch_are_held_apart() {
    // A home at its ceiling leaves a scratch with room, and the other way.
    assert_eq!(allows(k(0, 0), k(PRIVATE, 1)), Ok(()));
    assert_eq!(home_allows(k(HOME, 1), k(HOME, 1), (HOME, HOME), HOME), Ok(()));
}

/// Monero's wallet saves its cache whole beside the last: two copies of
/// about 21 MB, which its listing's home holds and an ordinary one refuses.
#[test]
fn a_larger_home_holds_a_wallets_two_copies() {
    let (copies, all) = (k(42 << 20, 3), (0, 42 << 20));
    assert_eq!(home_allows(k(21 << 20, 2), copies, all, HOME), Err(ENOSPC), "16 MiB");
    assert_eq!(home_allows(k(21 << 20, 2), copies, all, HOME_MOST), Ok(()), "64 MiB");
    assert_eq!(home_allows(k(0, 1), k(HOME_MOST + 1, 1), all, HOME_MOST), Err(ENOSPC));
    const { assert!(HOME_MOST + 4 * HOME <= HOMES, "the wallet's home beside four others") };
}
