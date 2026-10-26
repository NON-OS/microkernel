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

//! The restore's steps: a late answer never moves it, and it finishes only
//! on what vfs said, so a slow store never ends it early with defaults.

use crate::progress::{Progress, Say};
use crate::restore::stage::{next, Got, Stage};

const ALL: [Stage; 5] =
    [Stage::Settle, Stage::Marker, Stage::Answers, Stage::Values, Stage::Finished];

fn rng(s: &mut u64) -> u64 {
    (*s ^= *s << 13, *s ^= *s >> 7, *s ^= *s << 17, *s).3
}

#[test]
fn a_late_answer_leaves_every_stage_where_it_was() {
    for stage in ALL {
        assert_eq!(next(stage, Got::Late), stage, "{stage:?}");
    }
}

#[test]
fn every_answer_in_time_goes_forward_and_yes_finishes_in_four() {
    let mut stage = Stage::Settle;
    for want in [Stage::Marker, Stage::Answers, Stage::Values, Stage::Finished] {
        stage = next(stage, Got::Yes);
        assert_eq!(stage, want);
    }
    assert_eq!(next(Stage::Settle, Got::No), Stage::Settle, "a store not loaded proves nothing");
    assert_eq!(next(Stage::Marker, Got::No), Stage::Finished, "no marker: nothing was kept");
}

/* Any run of answers: finished only after the store loaded and a read was
 * answered in time; a run of late answers, however long, never finishes. */
#[test]
fn random_runs_finish_only_on_answers_in_time() {
    let mut s = 0x5eed_7001u64;
    for _ in 0..50_000 {
        let (mut stage, mut loaded, mut read) = (Stage::Settle, false, false);
        for _ in 0..rng(&mut s) % 16 {
            let got = [Got::Yes, Got::No, Got::Late][(rng(&mut s) % 3) as usize];
            let was = stage;
            stage = next(stage, got);
            loaded |= was == Stage::Settle && got == Got::Yes;
            read |= was != Stage::Settle && got != Got::Late;
            if stage == Stage::Finished {
                assert!(loaded && read, "finished from {was:?} on {got:?}");
            }
        }
    }
}

#[test]
fn progress_says_the_first_late_pass_once() {
    let mut p = Progress::new();
    assert_eq!(p.pass(false), Say::Nothing);
    assert_eq!(p.pass(true), Say::FirstLate(2));
    assert_eq!((p.pass(true), p.pass(false)), (Say::Nothing, Say::Nothing));
    assert_eq!((p.passes, p.first_late), (4, 2));
}
