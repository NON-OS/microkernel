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

//! A login needs a revocation list issued at most the stated age ago, 24
//! hours unless the table says otherwise, and the list must not name the
//! grant's id.

use crate::build::grant::GrantB;
use crate::build::revoke::RevB;
use crate::build::run::Setup;
use crate::build::{BASE_ID, HOUR, NOW};
use nsh_roles::Refusal;

fn with_list(rev: RevB, max_age_ms: Option<u64>) -> Result<(), Refusal> {
    let mut s = Setup { rev, ..Setup::default() };
    if let Some(ms) = max_age_ms {
        s.table.max_age_ms = ms;
    }
    s.login(&GrantB::default()).map(|_| ())
}

#[test]
fn a_list_older_than_24_hours_refuses_every_login() {
    let issued = |ms| RevB { issued_ms: ms, ..RevB::default() };
    assert_eq!(with_list(issued(NOW - 24 * HOUR), None), Ok(()));
    assert_eq!(with_list(issued(NOW - 24 * HOUR - 1), None), Err(Refusal::RevocationStale));
    assert_eq!(with_list(issued(NOW), None), Ok(()));
}

#[test]
fn a_list_from_the_future_is_not_fresh() {
    let r = RevB { issued_ms: NOW + 1, ..RevB::default() };
    assert_eq!(with_list(r, None), Err(Refusal::RevocationStale));
}

#[test]
fn the_age_the_table_states_is_the_age_held() {
    let issued = |ms| RevB { issued_ms: ms, ..RevB::default() };
    assert_eq!(with_list(issued(NOW - HOUR), Some(HOUR)), Ok(()));
    assert_eq!(with_list(issued(NOW - HOUR - 1), Some(HOUR)), Err(Refusal::RevocationStale));
    assert_eq!(with_list(issued(NOW - 3 * HOUR), Some(2 * HOUR)), Err(Refusal::RevocationStale));
}

#[test]
fn a_grant_on_the_list_is_refused_by_its_id() {
    let r = RevB { ids: vec![[0x10; 16], BASE_ID, [0x99; 16]], ..RevB::default() };
    assert_eq!(with_list(r, None), Err(Refusal::Revoked));
    let near = RevB { ids: vec![[0x10; 16], [0x12; 16]], ..RevB::default() };
    assert_eq!(with_list(near, None), Ok(()));
}
