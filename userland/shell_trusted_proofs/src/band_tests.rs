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

//! The chrome goes into the trusted band while a prompt, the system's prompt
//! or the hidden screen is up and back when none is; each move places it
//! first and takes it from the old band after.

use crate::state::attention::Attention;
use crate::state::trusted_rule::{in_band, left, moves, Band, Removed, CHROME_Z, TRUSTED_Z};

#[test]
fn the_chrome_is_in_the_band_exactly_while_something_asks() {
    assert!(!in_band(false, Attention::None));
    assert!(in_band(true, Attention::None), "a permission prompt");
    assert!(in_band(false, Attention::Prompt) && in_band(false, Attention::Hidden));
    assert_eq!((TRUSTED_Z, CHROME_Z), (0x4000_0000, 3_000_000), "the compositor's bands");
}

#[test]
fn each_move_places_before_it_takes() {
    assert_eq!(moves(false, true), Some((TRUSTED_Z, CHROME_Z)));
    assert_eq!(moves(true, false), Some((CHROME_Z, TRUSTED_Z)));
    assert_eq!(moves(true, true), None);
    assert_eq!(moves(false, false), None);
}

#[test]
fn no_terminal_opens_over_a_prompt() {
    use crate::state::trusted_rule::opens_terminal;
    assert!(opens_terminal(false), "Ctrl+Alt+T on a plain desktop opens the Terminal");
    assert!(!opens_terminal(true), "never over a prompt or the hidden screen");
}

/// A prompt closed, the chrome placed back under the trusted band, and its
/// removal from the band left unanswered: counted as gone, nothing took it
/// away and the router gave the shell every key and click for good.
/// Unanswered, the removal is asked again on the next sync.
#[test]
fn the_chrome_has_left_a_band_only_once_the_compositor_answered() {
    assert!(left(Removed::Taken));
    assert!(left(Removed::Refused), "an empty band: nothing there to take");
    assert!(!left(Removed::Unanswered));
    let back = Band::placed(false).taken(Removed::Unanswered);
    assert_eq!(back, Band { up: false, stray: true });
    assert_eq!(back.take(), Some(TRUSTED_Z), "the trusted band is asked again");
    assert_eq!(back.taken(Removed::Taken).take(), None);
}

#[test]
fn the_band_is_counted_left_only_by_the_rule() {
    const SYNC: &str = include_str!("../../capsule_desktop_shell/src/server/trusted_band.rs");
    assert!(SYNC.contains("ctx.trusted.band = Band::placed(want);"));
    assert!(SYNC.contains("ctx.trusted.band = ctx.trusted.band.place_unsure();"));
    assert!(SYNC.contains("ctx.trusted.band = ctx.trusted.band.taken(remove_band(port, rid, take));"));
    assert!(!SYNC.contains("let _ = remove_band"));
    const TICK: &str = include_str!("../../capsule_desktop_shell/src/server/runner/on_tick.rs");
    assert!(TICK.contains("crate::server::trusted_band::sync(ctx);"), "asked again each tick");
}
