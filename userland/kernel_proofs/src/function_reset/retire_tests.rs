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

//! A device leaving its holder: Bus Master off, grants taken,
//! BARs unmapped, ports quiet, the reset, DMA out of the IOMMU, the frames
//! freed, the domain detached, in that order on both release paths, and no
//! new claim until it is all done; and a teardown of the same holder during
//! the reset, from a thread that faulted on its revoked BAR, finds nothing
//! to do again.

use super::leaving::{claimable, take_all, take_held};
use super::retire::retire;
use super::retire_holder::{holding, Holder, PID};
use super::types::ClaimError;

const ORDER: [&str; 8] = ["bm", "take", "unmap", "quiet", "flr", "withdraw", "free", "detach"];

#[test]
fn both_paths_take_the_one_order() {
    let mut h = holding();
    assert_eq!(take_held(&mut h.claims, &mut h.busy, PID, 1), Ok(3));
    retire(&mut h);
    assert_eq!(h.log, ORDER);
    assert_eq!((h.mapped, h.freed), (false, 4), "never mapped back, freed once");
    let mut h = holding();
    assert_eq!(take_all(&mut h.claims, &mut h.busy, PID), vec![1]);
    retire(&mut h);
    assert_eq!(h.log, ORDER);
}

#[test]
fn a_fault_during_a_release_reset_tears_down_nothing_again() {
    let mut h = Holder { fault_during_reset: true, ..holding() };
    assert_eq!(take_held(&mut h.claims, &mut h.busy, PID, 1), Ok(3), "release takes it first");
    retire(&mut h);
    assert_eq!(h.freed, 4);
    let mut h = Holder { fault_during_reset: true, ..holding() };
    assert_eq!(take_all(&mut h.claims, &mut h.busy, PID), vec![1], "exit takes them all first");
    retire(&mut h);
}

#[test]
fn only_the_holder_takes_a_claim() {
    let Holder { mut claims, mut busy, .. } = holding();
    assert_eq!(take_held(&mut claims, &mut busy, 8, 1), Err(ClaimError::NotHolder));
    assert_eq!(take_held(&mut claims, &mut busy, PID, 2), Err(ClaimError::NotClaimed));
    assert_eq!((claims.len(), busy.len()), (1, 0), "a refused take leaves the claim, unmarked");
    assert_eq!(claimable(&claims, &busy, 1), Err(ClaimError::AlreadyClaimed));
}
