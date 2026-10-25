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

//! The desktop offers the installer once at the start of a live session. On
//! an installed system the policy store has restored Persistent before the
//! desktop starts, so it is never shown there; with no answer at all nothing
//! is shown, since there is no telling.

use crate::live_prompt::{LivePrompt, ASK_TICKS};

#[test]
fn a_live_session_shows_the_prompt() {
    assert_eq!(LivePrompt::new().step(Some(false)), LivePrompt::Showing);
}

#[test]
fn an_installed_session_never_shows_it() {
    let done = LivePrompt::new().step(Some(true));
    assert_eq!(done, LivePrompt::Done);
    assert_eq!(done.step(Some(false)), LivePrompt::Done);
}

#[test]
fn a_store_that_never_answers_shows_nothing() {
    let mut p = LivePrompt::new();
    for _ in 0..ASK_TICKS {
        assert!(!p.showing());
        p = p.step(None);
    }
    assert_eq!(p, LivePrompt::Done);
}

#[test]
fn a_late_answer_still_counts() {
    let p = LivePrompt::new().step(None).step(None).step(Some(false));
    assert_eq!(p, LivePrompt::Showing);
}

#[test]
fn once_dismissed_it_stays_down() {
    let shown = LivePrompt::new().step(Some(false));
    assert_eq!(shown.step(Some(false)), LivePrompt::Showing);
    assert_eq!(LivePrompt::Done.step(Some(false)), LivePrompt::Done);
}
