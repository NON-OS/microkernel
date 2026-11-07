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

//! The prompt that chooses or changes the passphrase.

use nonos_lock::rules::Refusal;

use crate::shell_lock::choose::{ChooseNote, ChooseStep, Chosen, Purpose};
use crate::shell_lock::Choose;

const ENTER: u32 = 0x0D;
const ESC: u32 = 0x1B;

fn typed(c: &mut Choose, text: &str) {
    for ch in text.chars() {
        assert_eq!(c.key(ch as u32), Chosen::Redraw);
    }
}

#[test]
fn a_first_passphrase_is_typed_twice_then_saved() {
    let mut c = Choose::new(Purpose::First);
    assert_eq!(c.step(), ChooseStep::New);
    typed(&mut c, "orchard lantern");
    assert_eq!(c.key(ENTER), Chosen::Redraw);
    assert_eq!(c.step(), ChooseStep::Again);
    typed(&mut c, "orchard lantern");
    assert_eq!(c.key(ENTER), Chosen::Save);
    assert_eq!(c.step(), ChooseStep::Saving);
    assert!(c.current.is_empty());
    assert_eq!(c.new.bytes(), b"orchard lantern");
    assert_eq!(c.key(b'x' as u32), Chosen::Nothing, "nothing typed while saving");
}

#[test]
fn a_short_one_is_refused_before_it_is_asked_again() {
    let mut c = Choose::new(Purpose::First);
    typed(&mut c, "short");
    c.key(ENTER);
    assert_eq!(c.step(), ChooseStep::New);
    assert_eq!(c.note(), ChooseNote::Refused(Refusal::Short));
}

#[test]
fn a_confirmation_that_differs_starts_the_new_one_again() {
    let mut c = Choose::new(Purpose::First);
    typed(&mut c, "orchard lantern");
    c.key(ENTER);
    typed(&mut c, "orchard lanterm");
    assert_eq!(c.key(ENTER), Chosen::Redraw);
    assert_eq!(c.step(), ChooseStep::New);
    assert_eq!(c.note(), ChooseNote::Refused(Refusal::Mismatch));
    assert!(c.new.is_empty() && c.again.is_empty(), "both wiped");
}

#[test]
fn changing_asks_the_current_one_first() {
    let mut c = Choose::new(Purpose::Change);
    assert_eq!(c.step(), ChooseStep::Current);
    assert_eq!(c.key(ENTER), Chosen::Nothing, "an empty current one does not move on");
    typed(&mut c, "the old one");
    c.key(ENTER);
    assert_eq!(c.step(), ChooseStep::New);
    typed(&mut c, "a new passphrase");
    c.key(ENTER);
    typed(&mut c, "a new passphrase");
    assert_eq!(c.enter(), Chosen::Save);
    assert_eq!(c.current.bytes(), b"the old one");
}

#[test]
fn a_wrong_current_one_wipes_every_field_and_asks_again() {
    let mut c = Choose::new(Purpose::Change);
    typed(&mut c, "guess");
    c.key(ENTER);
    typed(&mut c, "a new passphrase");
    c.key(ENTER);
    typed(&mut c, "a new passphrase");
    c.key(ENTER);
    c.refused(ChooseNote::Wrong);
    assert_eq!(c.step(), ChooseStep::Current);
    assert_eq!(c.note(), ChooseNote::Wrong);
    assert!(c.current.is_empty() && c.new.is_empty() && c.again.is_empty());
}

#[test]
fn escape_cancels() {
    let mut c = Choose::new(Purpose::First);
    typed(&mut c, "abc");
    assert_eq!(c.key(ESC), Chosen::Cancel);
}
