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

//! The chrome's move between the trusted band and its own, run against a
//! compositor that holds it in either band, as server/trusted_band.rs runs
//! it: a place may be taken and answered, taken with no answer coming back
//! (a busy compositor serves it after the shell's wait), or not taken; a
//! removal may be taken with no answer, or answered taken, or answered
//! refused for an empty band.
//!
//! The case found on a booted guest: a prompt up, the chrome placed in the
//! trusted band, its removal from its own band unanswered; then the prompt
//! answered before any repaint asked again. The shell still read the chrome
//! as under the band, so nothing took it from there: the router gave the
//! shell every key and click, and every other present of the chrome was
//! refused (its buffer shown in the trusted band too) and submitted whole.
//! The same came of a place in the trusted band served after the wait: the
//! shell read it as not placed and owed nothing, so the chrome stayed there
//! once the prompt was gone.

use crate::state::attention::Attention;
use crate::state::trusted_rule::{in_band, moves, Band, Removed, CHROME_Z, TRUSTED_Z};

/// Which bands hold the chrome: its own, the trusted one.
#[derive(Default)]
struct Compositor {
    own: bool,
    trusted: bool,
}

impl Compositor {
    fn band(&mut self, z: u32) -> &mut bool {
        if z == TRUSTED_Z {
            &mut self.trusted
        } else {
            assert_eq!(z, CHROME_Z);
            &mut self.own
        }
    }

    /// A place: taken and answered, taken with no answer, or lost.
    fn place(&mut self, z: u32, way: u8) -> bool {
        if way < 2 {
            *self.band(z) = true;
        }
        way == 0
    }

    /// A removal: taken and answered, taken with no answer, or lost.
    fn remove(&mut self, z: u32, way: u8) -> Removed {
        let held = *self.band(z);
        match way {
            0 => {
                *self.band(z) = false;
                if held {
                    Removed::Taken
                } else {
                    Removed::Refused
                }
            }
            1 => {
                *self.band(z) = false;
                Removed::Unanswered
            }
            _ => Removed::Unanswered,
        }
    }
}

/// server/trusted_band.rs sync, the calls made on `c`: a place, then the
/// removal the band still owes. `put` is how the place goes, `way` how the
/// removal goes.
fn sync_as(band: &mut Band, want: bool, c: &mut Compositor, put: u8, way: u8) {
    if let Some((place, _)) = moves(band.up, want) {
        if !c.place(place, put) {
            *band = band.place_unsure();
            return;
        }
        *band = Band::placed(want);
    }
    if let Some(take) = band.take() {
        *band = band.taken(c.remove(take, way));
    }
}

fn sync(band: &mut Band, want: bool, c: &mut Compositor, way: u8) {
    sync_as(band, want, c, 0, way);
}

fn held_where(band: Band, c: &mut Compositor) {
    let (here, other) = if band.up { (TRUSTED_Z, CHROME_Z) } else { (CHROME_Z, TRUSTED_Z) };
    assert!(*c.band(here), "the chrome is shown in the band the shell reads it in");
    if !band.stray {
        assert!(!*c.band(other), "nothing owed: the other band does not hold it");
    }
}

#[test]
fn a_prompt_answered_before_the_removal_was_does_not_leave_the_chrome_in_the_band() {
    let mut c = Compositor { own: true, trusted: false };
    let mut band = Band::default();
    // The prompt comes up; the removal from the chrome's own band is taken
    // but its answer is late.
    sync(&mut band, true, &mut c, 1);
    assert!(band.up && c.trusted && !c.own);
    // The prompt is answered before anything asks again.
    sync(&mut band, false, &mut c, 0);
    assert!(!band.up && !band.stray);
    assert!(c.own && !c.trusted, "the trusted band is empty again");
}

#[test]
fn a_lost_removal_is_asked_again_until_answered() {
    let mut c = Compositor { own: true, trusted: false };
    let mut band = Band::default();
    sync(&mut band, true, &mut c, 0);
    sync(&mut band, false, &mut c, 2);
    assert!(c.trusted && band.stray, "lost: still owed");
    sync(&mut band, false, &mut c, 2);
    assert!(c.trusted && band.stray);
    sync(&mut band, false, &mut c, 0);
    assert!(!c.trusted && !band.stray);
}

#[test]
fn a_place_in_the_band_served_after_the_wait_is_taken_back_once_the_prompt_is_gone() {
    let mut c = Compositor { own: true, trusted: false };
    let mut band = Band::default();
    // The lock screen comes up; the place is served after the shell's wait.
    sync_as(&mut band, true, &mut c, 1, 0);
    assert!(c.trusted && !band.up && band.stray, "maybe placed: owed");
    // The lock screen is answered before the next sync.
    sync(&mut band, false, &mut c, 0);
    assert!(!band.up && !band.stray);
    assert!(c.own && !c.trusted, "the trusted band is empty again");
}

#[test]
fn a_place_not_answered_is_asked_again_while_the_prompt_is_up() {
    let mut c = Compositor { own: true, trusted: false };
    let mut band = Band::default();
    sync_as(&mut band, true, &mut c, 2, 0);
    assert!(!c.trusted && !band.up && band.stray);
    sync_as(&mut band, true, &mut c, 0, 0);
    assert!(band.up && !band.stray && c.trusted && !c.own, "placed, its own band emptied");
}

#[test]
fn every_run_of_prompts_and_answers_ends_with_the_chrome_in_one_band() {
    // A small linear congruential generator: the same runs every time.
    let mut seed: u64 = 0x5eed_0935;
    let mut next = || {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (seed >> 33) as u32
    };
    for _ in 0..2000 {
        let mut c = Compositor { own: true, trusted: false };
        let mut band = Band::default();
        for _ in 0..24 {
            let prompt = next() % 2 == 0;
            let want = in_band(prompt, Attention::None);
            let put = (next() % 3) as u8;
            sync_as(&mut band, want, &mut c, put, (next() % 3) as u8);
            held_where(band, &mut c);
        }
        // The prompts end, and the compositor answers again.
        sync(&mut band, false, &mut c, (next() % 3) as u8);
        sync(&mut band, false, &mut c, 0);
        assert!(!band.up && !band.stray);
        assert!(c.own && !c.trusted, "no chrome left in the trusted band");
    }
}
