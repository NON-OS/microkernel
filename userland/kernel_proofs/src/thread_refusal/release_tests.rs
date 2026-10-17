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

//! A failed start puts back only its own claim (L77e): release_new stores
//! New only over the Ready its own claim_new made, so a kill since stands.

const CLAIM: &str = include_str!("../../../../src/process/core/table/claim.rs");

#[test]
fn a_failed_start_puts_back_only_its_own_claim() {
    let flat: String = CLAIM[CLAIM.find("pub fn release_new(").expect("release_new")..]
        .split_whitespace()
        .collect();
    let want = concat!(
        "letmutstate=pcb.state.lock();",
        "ifmatches!(*state,ProcessState::Ready){*state=ProcessState::New;}"
    );
    assert!(flat.contains(want), "release_new stores over a kill");
    assert_eq!(flat.matches("=ProcessState::New").count(), 1, "a second store");
}
