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

//! Delete asks first, and only a confirmed ask hands the item over.

use crate::delete_prompt::{DeletePrompt, DeleteTarget};

fn target(name: &str, is_dir: bool) -> DeleteTarget {
    DeleteTarget { name: name.into(), is_dir }
}

#[test]
fn delete_asks_and_deletes_nothing_yet() {
    let mut p = DeletePrompt::new();
    assert!(!p.showing());
    p.ask("notes.txt", false);
    assert!(p.showing());
    assert_eq!(p.target(), Some(&target("notes.txt", false)));
}

#[test]
fn confirming_hands_over_the_item_once() {
    let mut p = DeletePrompt::new();
    p.ask("Projects", true);
    assert_eq!(p.answer(true), Some(target("Projects", true)));
    assert!(!p.showing());
    assert_eq!(p.answer(true), None);
}

#[test]
fn cancelling_deletes_nothing_and_closes() {
    let mut p = DeletePrompt::new();
    p.ask("notes.txt", false);
    assert_eq!(p.answer(false), None);
    assert!(!p.showing());
    assert_eq!(p.answer(true), None);
}

#[test]
fn an_answer_with_nothing_asked_deletes_nothing() {
    assert_eq!(DeletePrompt::new().answer(true), None);
}

#[test]
fn a_second_ask_replaces_the_first() {
    let mut p = DeletePrompt::new();
    p.ask("a.txt", false);
    p.ask("b.txt", false);
    assert_eq!(p.answer(true), Some(target("b.txt", false)));
}
