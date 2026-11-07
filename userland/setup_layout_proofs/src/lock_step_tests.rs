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

//! Setup's lock screen step: the passphrase is typed twice and must meet
//! the rules; it is held only until the review sends it, and wiped then;
//! installing skips the step; nothing on the step is the passphrase itself.

use nonos_lock::rules::Refusal;

use crate::wizard_lock::{Keyed, LockStep, Stage};

const ENTER: u32 = 0x0D;
const ESC: u32 = 0x1B;
const BACK: u32 = 0x08;

fn typed(s: &mut LockStep, text: &str) {
    for c in text.chars() {
        assert_eq!(s.key(c as u32), Keyed::Stay);
    }
}

#[test]
fn typed_twice_the_same_is_chosen_and_moves_on() {
    let mut s = LockStep::new();
    assert_eq!(s.chosen(), None);
    typed(&mut s, "harbour light 7");
    assert_eq!(s.key(ENTER), Keyed::Stay);
    assert_eq!(s.stage, Stage::Again);
    typed(&mut s, "harbour light 7");
    assert_eq!(s.key(ENTER), Keyed::Advance);
    assert_eq!(s.chosen(), Some(&b"harbour light 7"[..]));
    assert_eq!(s.key(ENTER), Keyed::Advance, "shown again, Enter goes on");
}

#[test]
fn it_cannot_be_skipped() {
    let mut s = LockStep::new();
    assert_eq!(s.key(ENTER), Keyed::Stay, "an empty passphrase is refused");
    assert_eq!(s.refused, Some(Refusal::Short));
    typed(&mut s, "short");
    s.key(ENTER);
    assert_eq!(s.stage, Stage::New);
    assert_eq!(s.chosen(), None);
}

#[test]
fn a_confirmation_that_differs_starts_again_with_both_wiped() {
    let mut s = LockStep::new();
    typed(&mut s, "harbour light 7");
    s.key(ENTER);
    typed(&mut s, "harbour light 8");
    assert_eq!(s.key(ENTER), Keyed::Stay);
    assert_eq!(s.stage, Stage::New);
    assert_eq!(s.refused, Some(Refusal::Mismatch));
    assert!(s.new.is_empty() && s.again.is_empty());
}

#[test]
fn escape_retypes_or_goes_back_and_backspace_chooses_another() {
    let mut s = LockStep::new();
    assert_eq!(s.key(ESC), Keyed::Back);
    typed(&mut s, "harbour light 7");
    s.key(ENTER);
    typed(&mut s, "har");
    assert_eq!(s.key(ESC), Keyed::Stay);
    assert_eq!(s.stage, Stage::New);
    assert!(s.again.is_empty());
    s.key(ENTER);
    typed(&mut s, "harbour light 7");
    s.key(ENTER);
    assert_eq!(s.stage, Stage::Chosen);
    assert_eq!(s.key(BACK), Keyed::Stay);
    assert_eq!(s.stage, Stage::New);
    assert!(s.new.is_empty(), "choosing another wipes the one chosen");
}

#[test]
fn forget_wipes_both_copies() {
    let mut s = LockStep::new();
    typed(&mut s, "harbour light 7");
    s.key(ENTER);
    typed(&mut s, "harbour light 7");
    s.key(ENTER);
    s.forget();
    assert_eq!(s.chosen(), None);
    assert!(s.new.is_empty() && s.again.is_empty());
}

const RUNNER: &str = include_str!("../../capsule_setup_wizard/src/server/runner.rs");
const COMMIT: &str = include_str!("../../capsule_setup_wizard/src/render/screens/commit.rs");
const ORDER: &str = include_str!("../../capsule_setup_wizard/src/server/order.rs");

#[test]
fn the_step_comes_before_the_review_and_installing_skips_it() {
    assert!(ORDER.contains("pub const LOCK: u8 = 12;"));
    assert!(ORDER.contains("pub const REVIEW: u8 = 13;"));
    assert_eq!(crate::wizard_theme::STEP_LABELS[12], b"Lock screen");
    assert!(RUNNER.contains("if ctx.step == super::order::LOCK && screens::mode::keeps(&ctx) {"));
    let done = RUNNER.find("if ctx.step >= DONE {").unwrap();
    assert!(RUNNER[done..].starts_with("if ctx.step >= DONE {\n            ctx.lock.forget();"));
}

#[test]
fn the_review_sends_it_to_the_keyring_and_wipes_it() {
    let sent = COMMIT.find("crate::clients::keyring::set(chosen)").unwrap();
    let wiped = COMMIT.find("ctx.lock.forget();").unwrap();
    assert!(sent < wiped);
}
