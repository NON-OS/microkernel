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

//! A table's preamble, fleet and revocation age are each held to their
//! rule, and the table is read to its last byte and no further.

use super::policy::{open, parse};
use crate::build::table::TableB;
use nsh_roles::{PolicyError, MAX_REVOCATION_AGE_MS};
use nsh_wire::WireError;

#[test]
fn the_preamble_is_exact() {
    let good = TableB::default().bytes();
    let refused = |at: usize, v: u8, e: WireError| {
        let mut b = good.clone();
        b[at] = v;
        assert_eq!(parse(&b), Err(PolicyError::Wire(e)), "byte {at}");
    };
    refused(0, b'X', WireError::Magic);
    refused(4, 2, WireError::BodyVersion);
    refused(6, 1, WireError::Reserved);
    refused(7, 1, WireError::Reserved);
}

#[test]
fn a_zero_fleet_is_refused() {
    let t = TableB { fleet: [0; 16], ..TableB::default() };
    assert_eq!(open(&t), Err(PolicyError::Wire(WireError::Value)));
}

#[test]
fn the_revocation_age_is_from_one_ms_to_24_hours() {
    let age = |ms| open(&TableB { max_age_ms: ms, ..TableB::default() });
    assert_eq!(age(0), Err(PolicyError::Range));
    assert_eq!(age(1), Ok(()));
    assert_eq!(MAX_REVOCATION_AGE_MS, 24 * 3_600_000);
    assert_eq!(age(MAX_REVOCATION_AGE_MS), Ok(()));
    assert_eq!(age(MAX_REVOCATION_AGE_MS + 1), Err(PolicyError::Range));
}

#[test]
fn a_cut_or_padded_table_is_refused() {
    let good = TableB::default().bytes();
    assert_eq!(parse(&good[..good.len() - 1]), Err(PolicyError::Wire(WireError::Short)));
    let padded = [&good[..], &[0]].concat();
    assert_eq!(parse(&padded), Err(PolicyError::Wire(WireError::Trailing)));
}
