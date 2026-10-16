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

//! Ctrl+Alt+Delete: from the desktop or the prompt it shows the system's
//! prompt; the prompt locks or goes back; only the chord leaves the lock,
//! and no answer does.

use crate::state::attention::Attention;

#[test]
fn the_chord_opens_the_prompt_and_leaves_the_lock() {
    assert_eq!(Attention::None.chord(), Attention::Prompt);
    assert_eq!(Attention::Prompt.chord(), Attention::Prompt, "pressed twice: still the prompt");
    assert_eq!(Attention::Locked.chord(), Attention::None);
}

#[test]
fn the_prompt_locks_or_goes_back_and_nothing_else_leaves_the_lock() {
    assert_eq!(Attention::Prompt.answer(true), Attention::Locked);
    assert_eq!(Attention::Prompt.answer(false), Attention::None);
    assert_eq!(Attention::Locked.answer(false), Attention::Locked, "a click is not the chord");
    assert_eq!(Attention::Locked.answer(true), Attention::Locked);
    assert_eq!(Attention::None.answer(true), Attention::None, "no prompt, nothing to answer");
    assert!(Attention::Prompt.prompt() && Attention::Locked.locked());
    assert!(!Attention::None.prompt() && !Attention::None.locked());
}
