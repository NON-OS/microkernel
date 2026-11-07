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

//! A stalled EC tried again (track 8): after 1 s, then each back-off
//! doubled, at most TRIALS times. On trial one timeout turns it off again;
//! an answer keeps it on. A flooding EC is never tried again, nor one that
//! spent its budget of timeouts.

use crate::ec::{read, Health, Off, FIRST_BACKOFF_MS, STRIKES, TRIALS};
use crate::ec_model::{EcModel, PORTS};

fn stalled() -> Health {
    let mut h = Health::default();
    for _ in 0..STRIKES {
        h.note::<()>(&Err(crate::ec::EcError::Timeout));
    }
    assert_eq!(h.off, Some(Off::Stalled));
    h
}

#[test]
fn a_stall_is_tried_again_after_its_back_off() {
    let mut h = stalled();
    assert!(!h.retry(10_000), "the back-off starts when the stall is first seen");
    assert!(!h.retry(10_000 + FIRST_BACKOFF_MS - 1));
    assert!(h.retry(10_000 + FIRST_BACKOFF_MS) && h.usable() && h.trials() == 1);
}

#[test]
fn on_trial_one_timeout_turns_it_off_and_an_answer_keeps_it_on() {
    let mut ec = EcModel::new(0);
    ec.stuck = true;
    let mut h = stalled();
    h.retry(0);
    assert!(h.retry(FIRST_BACKOFF_MS));
    assert!(h.note(&read(&PORTS, &mut ec, 0x10)), "off again at the first timeout");
    assert_eq!(ec.slept_ms, 1000, "one second a trial");
    h.retry(5_000);
    assert!(!h.retry(5_000 + 2 * FIRST_BACKOFF_MS - 1), "the back-off doubled");
    assert!(h.retry(5_000 + 2 * FIRST_BACKOFF_MS));
    ec.stuck = false;
    h.note(&read(&PORTS, &mut ec, 0x10));
    assert!(h.usable(), "answered: on");
}

#[test]
fn the_trials_end_and_a_flood_is_never_tried() {
    let mut h = stalled();
    let mut now = 0;
    for i in 0..TRIALS {
        h.retry(now);
        now += FIRST_BACKOFF_MS << i;
        assert!(h.retry(now), "trial {i}");
        h.note::<()>(&Err(crate::ec::EcError::Timeout));
    }
    assert!(!h.retry(u64::MAX) && h.off == Some(Off::Stalled), "off for the boot");
    assert_eq!(now, FIRST_BACKOFF_MS * ((1 << TRIALS) - 1), "63 s of back-off in all");
    let mut f = Health::default();
    assert!(f.flooded());
    f.retry(0);
    assert!(!f.retry(u64::MAX));
}

/// An EC that answers just often enough to clear its strikes still stops
/// costing time: BUDGET timeouts in a boot turn it off for good.
#[test]
fn the_boots_timeouts_are_bounded_whatever_the_pattern() {
    use crate::ec::{EcError, BUDGET};
    let mut h = Health::default();
    let mut turned = 0;
    for _ in 0..BUDGET {
        turned += u32::from(h.note::<()>(&Err(EcError::Timeout)));
        h.note(&Ok(()));
    }
    assert_eq!((h.off, h.timeouts(), turned), (Some(Off::Spent), BUDGET, 1), "said once");
    assert!(!h.retry(u64::MAX), "never tried again");
    let mut s = stalled();
    for _ in u16::from(STRIKES)..BUDGET {
        s.retry(0);
        s.retry(u64::MAX >> 1);
        s.note::<()>(&Err(EcError::Timeout));
    }
    assert_eq!(s.off, Some(Off::Spent), "trials and all, the budget holds");
}
