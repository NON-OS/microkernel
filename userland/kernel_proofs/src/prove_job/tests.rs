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

use super::job_argv::{job_argv, JobKind, JOB_ARG};
use super::rate::Rate;

const MIN: u64 = 60 * 1000;

#[test]
fn one_delivery_per_measurement_per_ten_minutes() {
    let mut r = Rate::new();
    let (linux, other) = ([0x11; 32], [0x22; 32]);
    assert!(r.take(&linux, 5 * MIN));
    assert!(!r.take(&linux, 5 * MIN + 1), "at once again");
    assert!(!r.take(&linux, 15 * MIN - 1), "a minute short");
    assert!(r.take(&other, 6 * MIN), "another measurement is its own");
    assert!(r.take(&linux, 15 * MIN), "ten minutes on");
    assert!(!r.take(&linux, 16 * MIN));
}

#[test]
fn a_clock_at_zero_still_holds_the_rate_and_the_oldest_gives_way() {
    let mut r = Rate::new();
    assert!(r.take(&[1; 32], 0));
    assert!(!r.take(&[1; 32], 0), "a first take at time 0 still counts");
    for k in 2..=9u8 {
        assert!(r.take(&[k; 32], u64::from(k) * MIN));
    }
    assert!(r.take(&[1; 32], 10 * MIN), "evicted, then free again");
    assert!(!r.take(&[9; 32], 10 * MIN), "the newest is kept");
}

#[test]
fn the_job_is_told_its_op_and_its_pid() {
    let a = job_argv(JobKind::Deliver, 4242);
    assert_eq!(a, ["--nonos-prove-job", "deliver", "4242"]);
    assert_eq!(job_argv(JobKind::Prove, 7)[1], "prove");
    assert_eq!(a[0], JOB_ARG);
}
