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

//! A device retiring is busy: no new claim takes it until its
//! retire returns, and a claim's own mark coming off does not end the mark
//! of a release that began meanwhile.

use super::leaving::{claimable, done, take_held};
use super::retire::retire;
use super::retire_holder::{holding, PID};
use super::types::ClaimError;

#[test]
fn a_retiring_device_is_busy_until_retire_returns() {
    let mut h = holding();
    assert_eq!(take_held(&mut h.claims, &mut h.busy, PID, 1), Ok(3));
    assert_eq!(claimable(&h.claims, &h.busy, 1), Err(ClaimError::Busy));
    retire(&mut h);
    assert_eq!(claimable(&h.claims, &h.busy, 1), Ok(()), "claimable once it is all done");
}

#[test]
fn a_claims_own_mark_leaves_a_release_begun_meanwhile_marked() {
    // The claim is published while its mark is still on; a thread of the
    // holder releases it at once; the claim then takes its mark off.
    let mut h = holding();
    h.busy.push(1);
    assert_eq!(take_held(&mut h.claims, &mut h.busy, PID, 1), Ok(3));
    done(&mut h.busy, 1);
    assert_eq!(claimable(&h.claims, &h.busy, 1), Err(ClaimError::Busy), "still retiring");
    retire(&mut h);
    assert_eq!(claimable(&h.claims, &h.busy, 1), Ok(()));
}
