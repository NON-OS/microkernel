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

/*
 * Waiting for the Anyone network before a download: go once net.anon says
 * it is ready (or is too old to say), ask again a second later while it
 * builds its circuit, give up after three minutes; the status answer read
 * whole or not at all; the line says the step.
 */

use crate::anyone_wait::{decide, line, read, Heard, Wait, HOLD_MS, STATUS_ASK};

const READY: Heard = Heard { ready: true, step: 5, steps: 5 };
const STEP3: Heard = Heard { ready: false, step: 3, steps: 5 };

#[test]
fn the_status_answer_is_read_whole() {
    assert_eq!(STATUS_ASK, 6);
    assert_eq!(read(&[3, 1, 1, 5, 5]), Some(READY));
    assert_eq!(read(&[3, 1, 0, 3, 5]), Some(STEP3));
    for bad in [&[3u8, 1, 0, 3][..], &[3, 2, 0, 3, 5], &[4, 1, 0, 3, 5], &[3, 1, 2, 3, 5], &[3, 1, 0, 6, 5], &[3, 1, 0, 0, 5], &[3, 1, 0, 1, 9]] {
        assert_eq!(read(bad), None, "{bad:?}");
    }
}

#[test]
fn a_ready_network_goes_and_a_building_one_is_waited_for() {
    assert_eq!(decide(true, Some(READY), true, 0), Wait::Go);
    assert_eq!(decide(true, Some(STEP3), true, 1_000), Wait::Again(Some(STEP3)));
    /* Not registered yet: waited for, with no step to say. */
    assert_eq!(decide(false, None, false, 1_000), Wait::Again(None));
    /* Registered and silent (a long call of its own): waited for. */
    assert_eq!(decide(true, None, false, 1_000), Wait::Again(None));
    /* Registered and answering something else: older than the ask, so go. */
    assert_eq!(decide(true, None, true, 0), Wait::Go);
}

#[test]
fn three_minutes_is_the_most_a_download_waits() {
    assert_eq!(decide(true, Some(STEP3), true, HOLD_MS - 1), Wait::Again(Some(STEP3)));
    assert_eq!(decide(true, Some(STEP3), true, HOLD_MS), Wait::GiveUp);
    assert_eq!(decide(false, None, false, HOLD_MS), Wait::GiveUp);
    /* Ready at the last moment still goes. */
    assert_eq!(decide(true, Some(READY), true, HOLD_MS), Wait::Go);
    assert_eq!(HOLD_MS, 180_000);
}

#[test]
fn the_line_says_the_step() {
    assert_eq!(line(Some(STEP3)), "Anyone is building its circuit, step 3 of 5");
    assert_eq!(line(None), "Anyone is starting");
}

/* Anyone not up in time is its own reason, with Retry and the direct choice. */
#[test]
fn anyone_not_up_is_its_own_reason() {
    use crate::install::fetch_exit::ANYONE_NOT_UP;
    use crate::install::model_dep::fetched;
    use crate::install::why::Why;
    use crate::store::progress::Progress;
    assert_eq!(fetched(i64::from(ANYONE_NOT_UP)), Err(Why::AnyoneNotUp));
    let p = Progress::Failed(Why::AnyoneNotUp.code() as u8);
    assert!(p.retryable());
    let said = String::from_utf8_lossy(p.sentence().unwrap().0).into_owned();
    assert!(said.starts_with("Anyone did not build a circuit within 3 minutes; retry, or press d"), "{said}");
    assert!(crate::store::route_offer::ROUTE_REASONS.contains(&33));
}
