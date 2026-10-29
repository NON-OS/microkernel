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

//! The name for the core's first start: taken only when asked, only as the
//! kernel admits the word, and typed into its own field, which starts empty.

use crate::model::focus::{Focus, Key};
use crate::model::start::{Phase, Start, EVERY_MS, LOOK_MS};
use crate::model::start_name::{word, NAME_MAX, WORD_MAX};
use crate::model::state::Model;
use crate::start_tests::launch_at;

#[test]
fn a_name_is_taken_only_when_asked_and_only_as_the_kernel_admits_it() {
    let mut s = Start::new();
    assert!(!s.named("Ada"), "not asked yet");
    assert_eq!(word("Ada").map(|w| w.len()), Some(23));
    assert_eq!(word(&"x".repeat(NAME_MAX)).map(|w| w.len()), Some(WORD_MAX));
    assert_eq!(word(&"x".repeat(NAME_MAX + 1)), None);
    assert_eq!(word("   "), None);
    assert_eq!(word("A\u{0}b"), None);
    assert_eq!(word("A\nb"), None);
}

#[test]
fn the_name_field_takes_keys_alone_and_enter_gives_it() {
    let mut m = Model::new();
    m.focus = Focus::Name;
    assert!(m.name.is_empty(), "no name is put in for the person");
    for ch in "Ada".chars() {
        assert!(m.on_key(Key::Char(ch)));
    }
    assert!(!m.on_key(Key::Tab) && !m.on_key(Key::Search));
    assert!(m.on_key(Key::Backspace) && m.name == "Ad");
    assert!(m.compose.is_empty() && m.search.is_empty());
    assert!(m.on_key(Key::Enter));
    assert_eq!(m.focus, Focus::Named);
    m.focus = Focus::Name;
    m.insert(&"y".repeat(NAME_MAX + 9));
    assert_eq!(m.name.len(), NAME_MAX);
}

#[test]
fn a_named_core_that_never_listens_stops_and_a_refusal_stops() {
    let mut s = Start::new();
    let (_, at) = launch_at(&mut s, 0, 0);
    s.tick(at + LOOK_MS);
    assert!(s.named("Ada"));
    let (_, at) = launch_at(&mut s, at + LOOK_MS + EVERY_MS, 0);
    s.tick(at + LOOK_MS);
    assert!(matches!(s.phase, Phase::Stopped(_)));
    let mut r = Start::new();
    launch_at(&mut r, 0, -1);
    assert!(matches!(r.phase, Phase::Stopped(_)));
}
