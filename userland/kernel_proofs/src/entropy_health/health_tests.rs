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

//! The repetition count test, the cutoffs and the startup.

use super::bytes::uniform;
use super::health::*;

/// 1 + CRITBINOM(n, p, 1 - alpha): the least count whose upper tail is
/// below alpha, plus one.
fn apt_cutoff(n: u32, p: f64, alpha: f64) -> u32 {
    let ln_choose = |k: u32| -> f64 { (1..=k).map(|i| ((n - k + i) as f64 / i as f64).ln()).sum() };
    let mut below = 0.0;
    for k in 0..=n {
        let pk = (ln_choose(k) + k as f64 * p.ln() + (n - k) as f64 * (1.0 - p).ln()).exp();
        below += pk;
        if below >= 1.0 - alpha {
            return 1 + k;
        }
    }
    unreachable!()
}

#[test]
fn the_cutoffs_follow_from_the_claimed_entropy() {
    let h = CLAIMED_BITS as f64;
    assert_eq!(RCT_CUTOFF, 1 + (20.0 / h).ceil() as u32);
    assert_eq!(APT_CUTOFF, apt_cutoff(APT_WINDOW, 2f64.powf(-h), 2f64.powi(-20)));
    assert_eq!((RCT_CUTOFF, APT_CUTOFF), (6, 62));
}

#[test]
fn a_run_one_short_of_the_cutoff_passes_and_the_cutoff_alarms() {
    let mut h = Health::new();
    for _ in 1..RCT_CUTOFF {
        assert_eq!(h.sample(7), Ok(()));
    }
    assert_eq!(h.sample(8), Ok(()), "the run ends");
    for _ in 1..RCT_CUTOFF {
        assert_eq!(h.sample(9), Ok(()));
    }
    assert_eq!(h.sample(9), Err(Fault::Repetition));
}

#[test]
fn uniform_bytes_pass_and_prove_the_source() {
    let mut h = Health::new();
    for (i, x) in uniform(1, 1 << 16).into_iter().enumerate() {
        assert_eq!(h.sample(x), Ok(()), "sample {i}");
        assert_eq!(h.proven(), i + 1 >= STARTUP as usize);
    }
}

#[test]
fn an_alarm_unproves_the_source() {
    let mut h = Health::new();
    uniform(2, STARTUP as usize).into_iter().for_each(|x| h.sample(x).unwrap());
    assert!(h.proven());
    (0..RCT_CUTOFF).for_each(|_| _ = h.sample(0));
    assert!(!h.proven(), "an alarm restarts the startup");
}
