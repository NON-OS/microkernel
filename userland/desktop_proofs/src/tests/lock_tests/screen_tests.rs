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

//! The lock screen's keys and the keyring's answers.

use nonos_lock::field::Field;
use crate::shell_lock::screen::{Asked, Checked, LockScreen, Note, Phase};

const ENTER: u32 = 0x0D;
const ESC: u32 = 0x1B;
const BACK: u32 = 0x08;

fn typed(s: &mut LockScreen, text: &str) {
    for c in text.chars() {
        assert_eq!(s.key(c as u32, 0), Asked::Redraw);
    }
}

#[test]
fn locking_covers_with_an_empty_field() {
    let mut s = LockScreen::default();
    assert!(!s.covers());
    assert_eq!(s.key(b'a' as u32, 0), Asked::Nothing, "no field while open");
    s.lock();
    assert!(s.covers());
    assert_eq!(s.phase(), Phase::Locked);
    assert!(s.field.is_empty());
}

#[test]
fn enter_sends_the_field_once_and_only_when_typed() {
    let mut s = LockScreen::default();
    s.lock();
    assert_eq!(s.key(ENTER, 0), Asked::Nothing, "an empty field is not sent");
    typed(&mut s, "lantern orchard");
    assert_eq!(s.field.bytes(), b"lantern orchard");
    assert_eq!(s.key(ENTER, 0), Asked::Check);
    assert_eq!(s.phase(), Phase::Checking);
    assert_eq!(s.key(b'x' as u32, 0), Asked::Nothing, "nothing typed while checking");
    assert_eq!(s.key(ENTER, 0), Asked::Nothing, "and nothing sent twice");
}

#[test]
fn the_right_passphrase_opens_and_wipes_the_field() {
    let mut s = LockScreen::default();
    s.lock();
    typed(&mut s, "lantern orchard");
    s.key(ENTER, 0);
    s.answered(Checked::Right, 10);
    assert!(!s.covers());
    assert!(s.field.is_empty());
    assert_eq!(s.note(), Note::None);
}

#[test]
fn a_wrong_one_stays_locked_says_so_and_holds_the_next_try() {
    let mut s = LockScreen::default();
    s.lock();
    typed(&mut s, "not it");
    s.key(ENTER, 0);
    s.answered(Checked::Wrong { failures: 4, wait_ms: 2000 }, 100);
    assert_eq!(s.phase(), Phase::Locked);
    assert!(s.field.is_empty(), "a wrong entry is wiped");
    assert_eq!(s.note(), Note::Wrong { failures: 4 });
    assert_eq!(s.wait_left(100), 2000);
    typed(&mut s, "again");
    assert_eq!(s.key(ENTER, 1500), Asked::Nothing, "not sent while the wait runs");
    assert_eq!(s.key(ENTER, 2100), Asked::Check);
}

#[test]
fn no_answer_stays_locked() {
    let mut s = LockScreen::default();
    s.lock();
    typed(&mut s, "anything");
    s.key(ENTER, 0);
    s.answered(Checked::Unavailable, 0);
    assert!(s.covers());
    assert_eq!(s.note(), Note::Unavailable);
}

#[test]
fn a_wait_read_at_start_holds_the_first_try() {
    let mut s = LockScreen::default();
    s.hold_for(8000, 50);
    s.lock();
    typed(&mut s, "anything");
    assert_eq!(s.key(ENTER, 60), Asked::Nothing);
    assert_eq!(s.key(ENTER, 8050), Asked::Check);
}

#[test]
fn backspace_escape_and_characters_edit_the_field() {
    let mut s = LockScreen::default();
    s.lock();
    typed(&mut s, "ab");
    s.key('é' as u32, 0);
    assert_eq!(s.field.chars(), 3);
    s.key(BACK, 0);
    assert_eq!(s.field.bytes(), b"ab");
    s.key(ESC, 0);
    assert!(s.field.is_empty());
    assert_eq!(s.key(0x7F, 0), Asked::Nothing);
    assert_eq!(s.key(0x01, 0), Asked::Nothing);
}

#[test]
fn a_field_holds_at_most_256_bytes() {
    let mut f = Field::default();
    for _ in 0..256 {
        assert!(f.push('a'));
    }
    assert!(!f.push('a'));
    assert!(!f.push('é'));
    f.pop();
    assert!(!f.push('é'), "two bytes do not fit in one");
    assert!(f.push('b'));
    assert_eq!(f.bytes().len(), 256);
}

#[test]
fn locking_again_keeps_what_was_said() {
    let mut s = LockScreen::default();
    s.lock();
    typed(&mut s, "x");
    s.key(ENTER, 0);
    s.answered(Checked::Wrong { failures: 1, wait_ms: 0 }, 0);
    s.lock();
    assert_eq!(s.note(), Note::Wrong { failures: 1 });
}
