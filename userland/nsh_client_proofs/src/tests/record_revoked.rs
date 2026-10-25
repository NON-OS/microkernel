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

//! A withdrawn enrolment is refused offline as well as online: the check
//! needs no clock, only the policy's list.

use super::record_good::report;
use crate::kit::attest::TestAttester;
use crate::kit::enrol::EnrolSpec;
use crate::kit::pair::policy;
use crate::kit::record::Rec;
use nonos_hash::sha512;
use nsh_client::{Bound, EnrolRefusal, HostRefusal, RecordRefusal};

#[test]
fn a_record_under_a_withdrawn_enrolment_is_refused() {
    let rec = Rec::good(10).bytes();
    let a = TestAttester::default();
    let mut p = policy();
    assert!(report(&rec, &[], &Bound::default(), &p, &a).is_ok());
    p.revoked = vec![[7; 64], sha512(&EnrolSpec::good().bytes())];
    let r = report(&rec, &[], &Bound::default(), &p, &a);
    assert_eq!(r, Err(RecordRefusal::Host(HostRefusal::Enrol(EnrolRefusal::Revoked))));
}
