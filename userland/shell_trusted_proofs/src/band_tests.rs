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
use crate::state::trusted_rule::{in_band, moves, CHROME_Z, TRUSTED_Z};

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
