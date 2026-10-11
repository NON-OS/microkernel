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

//! The meter's jobs (v1.4.14): one at a time, each result once, no id twice.

use nonos_nox_meter::job::Jobs;
use nonos_payment_proto::{EAGAIN, EBUSY, ENOENT, ETIME};

#[test]
fn one_job_at_a_time_and_its_result_once() {
    let mut j = Jobs::default();
    let a = j.start().unwrap();
    assert_eq!((j.start(), j.running()), (Err(EBUSY), Some(a)));
    assert_eq!(j.take(a), Err(EAGAIN));
    j.finish(a + 9, Ok(vec![1]));
    assert_eq!(j.take(a), Err(EAGAIN), "another id finishes nothing");
    j.finish(a, Ok(vec![7, 7]));
    assert_eq!(j.running(), None);
    assert_eq!(j.take(a), Ok(vec![7, 7]));
    assert_eq!(j.take(a), Err(ENOENT), "once");
}

#[test]
fn ids_are_never_reused_and_a_stale_one_is_enoent() {
    let mut j = Jobs::default();
    let a = j.start().unwrap();
    j.finish(a, Err(ETIME));
    let b = j.start().unwrap();
    assert!(b > a);
    assert_eq!(j.take(a), Err(ENOENT), "the older result was let go");
    j.finish(b, Err(ETIME));
    assert_eq!(j.take(b), Err(ETIME), "a status is the result too");
    assert_eq!(j.take(0), Err(ENOENT));
}
