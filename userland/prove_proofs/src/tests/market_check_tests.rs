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

//! the plan step 5 before proving: the policy names the record's app, and a
//! market policy names this listing; anything else is POLICY.

use nonos_proof_abi::wire::POLICY;

use crate::fixture::{hist, policy, record, LISTING};
use crate::market::check::check;

const ALLOWED: [usize; 4] = [0, 1, 60, 231];

#[test]
fn the_policy_must_name_the_records_app_and_this_listing() {
    let p = policy(&LISTING, &ALLOWED);
    let rec = record(&p, &hist(), &[]);
    assert_eq!(check(&p, &LISTING, &rec), Ok(()));
    assert_eq!(check(&p, &[0x4D; 32], &rec), Err(POLICY), "another listing");
    let vendor = policy(&[0; 32], &ALLOWED);
    let vrec = record(&vendor, &hist(), &[]);
    assert_eq!(check(&vendor, &[0x4D; 32], &vrec), Ok(()), "a vendor policy names none");
    let wider = policy(&LISTING, &[0, 1, 60, 231, 101]);
    assert_eq!(check(&wider, &LISTING, &rec), Err(POLICY), "another app's policy");
}
