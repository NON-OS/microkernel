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

//! G4 (L81b): a reused pid is forgotten, its writer put right, under the
//! rows taken the responsive way.

use crate::proofs::proofs_8::transport_takeover::text::{between, code};

const BACK: &str = include_str!("../../../../src/services/lifecycle/trip_back.rs");

#[test]
fn g4_a_reused_pid_is_forgotten_with_its_writer_put_right() {
    let forget = concat!(
        "pub fn forget(&self, pid: u32) { let _quiet = super::rest_mark::quiet();",
        " let mut rows = crate::smp::lock_responsive(&self.rows);",
        " if rows.sweep(|p| p == pid) { self.left(true); } } }",
    );
    assert_eq!(between(BACK, "pub fn forget(", "pub(super) fn asked_again("), code(forget));
}
