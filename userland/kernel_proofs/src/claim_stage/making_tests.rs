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

//! A claim as its call first enters it (`Claim::making`): being made, with
//! nothing inside it and nothing leaving it.

use super::types::{Claim, Stage};

#[test]
fn a_claim_is_made_empty_and_in_the_making() {
    let c = Claim::making(7, 0x10, 3);
    assert_eq!((c.pid, c.device_id, c.epoch), (7, 0x10, 3));
    assert_eq!((c.stage, c.inflight, c.leavers), (Stage::Making, 0, 0));
}
