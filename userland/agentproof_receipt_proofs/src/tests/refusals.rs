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
//! A whole receipt verifies, and says what it carries.
//! Each check refuses by its own name when the one thing it holds changes.

use agentproof_receipt::verify;
use nonos_proof_abi::{FLAG_FINAL, FLAG_OVERFLOW, FLAG_SATURATED};

use crate::fixture::Fixture;

fn refused(f: &mut Fixture) -> &'static str {
    let (bytes, trust) = (f.bytes(), f.trust());
    verify(&bytes, &trust).unwrap_err().name()
}

#[test]
fn the_record_must_be_a_finished_seal_by_a_trusted_supervisor() {
    for flags in [0, FLAG_FINAL | FLAG_SATURATED, FLAG_FINAL | FLAG_OVERFLOW] {
        let mut f = Fixture::new(b"strict");
        f.record.flags = flags;
        assert_eq!(refused(&mut f), "unfinished");
    }
    let mut f = Fixture::new(b"strict");
    f.record.authority = 1;
    assert_eq!(refused(&mut f), "supervisor");
    let mut f = Fixture::new(b"strict");
    f.record.sup_caps |= 1 << 2;
    assert_eq!(refused(&mut f), "supervisor", "the supervisor's caps are part of its trust");
}

#[test]
fn slot_0_must_be_this_app_in_this_box() {
    let mut f = Fixture::new(b"strict");
    f.box_word = b"offline".to_vec();
    assert_eq!(refused(&mut f), "binding", "another box");
    let mut f = Fixture::new(b"strict");
    f.policy = nonos_linux_policy::unlisted([8; 32]).encode();
    assert_eq!(refused(&mut f), "binding", "another program");
    let mut f = Fixture::new(b"strict");
    f.box_word = b"loose".to_vec();
    assert_eq!(refused(&mut f), "box");
    let mut f = Fixture::new(b"strict");
    f.policy[0] ^= 1;
    assert_eq!(refused(&mut f), "policy");
}
