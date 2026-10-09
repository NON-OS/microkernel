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

//! The calculator's keys do what their labels say, on the real window state
//! and the real actions: every digit key on every keypad, the hex letters
//! typed in Programmer mode, a factorial too large to hold, and the History
//! page reaching every calculation the ring keeps.

use crate::calc::actions::dispatch;
use crate::calc::buttons::{grid, Action};
use crate::calc::event::key_classifier::{classify, Classified};
use crate::calc::fixed::FRAC;
use crate::calc::history::CAP;
use crate::calc::manifest::HEIGHT;
use crate::calc::mode::{Mode, MODES};
use crate::calc::prog::Base;
use crate::calc::sci::{apply, SciFn};
use crate::calc::state::{ErrorKind, State};
use crate::calc::ui::history_geom::{capacity, entry_at, max_scroll, scroll_by};

/// Every key whose label is digits enters exactly those digits. The "00" key
/// entered one zero, so 7 then 00 read 70.
#[test]
fn every_digit_key_enters_the_digits_on_its_label() {
    for mode in [Mode::Basic, Mode::Scientific] {
        for row in grid(mode) {
            for button in row.iter() {
                if !button.label.bytes().all(|b| b.is_ascii_digit()) {
                    continue;
                }
                let mut state = State::new();
                state.set_mode(mode);
                dispatch::run(&mut state, Action::Digit(7));
                dispatch::run(&mut state, button.action);
                let typed: i128 = format!("7{}", button.label).parse().unwrap();
                assert_eq!(state.display, typed * FRAC, "{} in {}", button.label, mode.label());
            }
        }
    }
}

#[test]
fn the_double_zero_key_multiplies_the_entry_by_a_hundred() {
    let mut state = State::new();
    dispatch::run(&mut state, Action::Digit(4));
    dispatch::run(&mut state, Action::Digit(2));
    dispatch::run(&mut state, Action::DoubleZero);
    assert_eq!(state.display, 4200 * FRAC);
}

/// 21! is a whole number the readout can show; it used to be refused as "Not
/// defined". Past 28! it is too large, and the readout says that instead.
#[test]
fn a_factorial_is_refused_only_for_what_it_really_is() {
    assert_eq!(apply(SciFn::Factorial, 21 * FRAC), Ok(51_090_942_171_709_440_000 * FRAC));
    assert!(apply(SciFn::Factorial, 28 * FRAC).is_ok());
    assert_eq!(apply(SciFn::Factorial, 29 * FRAC), Err(ErrorKind::Overflow));
    assert_eq!(apply(SciFn::Factorial, 1_000_000_000 * FRAC), Err(ErrorKind::Overflow));
    assert_eq!(apply(SciFn::Factorial, FRAC / 2), Err(ErrorKind::DomainError));
    assert_eq!(apply(SciFn::Factorial, -FRAC), Err(ErrorKind::DomainError));
}

fn key(state: &mut State, ch: u8) {
    if let Classified::Action(action) = classify(ch as u32, state.mode) {
        dispatch::run(state, action);
    }
}

/// In Programmer mode the letters are hex digits, as the keypad's A to F are;
/// "c" cleared the entry there before.
#[test]
fn hex_letters_type_hex_digits_in_programmer_mode() {
    let mut state = State::new();
    state.set_mode(Mode::Programmer);
    dispatch::run(&mut state, Action::SetBase(Base::Hex));
    for ch in *b"1cAfE" {
        key(&mut state, ch);
    }
    assert_eq!(state.prog, 0x1CAFE);
    // Outside Programmer mode the same keys keep their own meanings.
    assert!(matches!(classify(b'c' as u32, Mode::Basic), Classified::Action(Action::Clear)));
    assert!(matches!(classify(b'a' as u32, Mode::Basic), Classified::Action(Action::MemoryAdd)));
}

#[test]
fn a_hex_letter_is_refused_in_a_base_without_it() {
    let mut state = State::new();
    state.set_mode(Mode::Programmer);
    key(&mut state, b'5');
    key(&mut state, b'c');
    assert_eq!(state.prog, 5);
}

fn full_ring() -> State {
    let mut state = State::new();
    for i in 0..CAP as i128 {
        state.history.push(b"1 + 1", i * FRAC);
    }
    state
}

/// The ring keeps 32 calculations and the page shows fewer; scrolled to the
/// end, the oldest one is on the last row, so none is out of reach.
#[test]
fn the_history_page_reaches_every_calculation_it_keeps() {
    let state = full_ring();
    let rows = capacity(HEIGHT as i32);
    assert!(rows > 0 && rows < CAP, "{rows} rows");
    let end = scroll_by(0, i32::MAX, state.history.len(), HEIGHT as i32);
    assert_eq!(end, max_scroll(CAP, HEIGHT as i32));
    let oldest = state.history.get(entry_at(end, rows - 1)).map(|e| e.value);
    assert_eq!(oldest, Some(0), "the first calculation pushed is the oldest");
    assert!(state.history.get(entry_at(end, rows)).is_none());
}

#[test]
fn history_scrolling_stays_in_range() {
    let len = CAP;
    let h = HEIGHT as i32;
    assert_eq!(scroll_by(0, -3, len, h), 0);
    assert_eq!(scroll_by(2, -1, len, h), 1);
    assert_eq!(scroll_by(max_scroll(len, h), 5, len, h), max_scroll(len, h));
    // A page that fits everything does not scroll at all.
    assert_eq!(scroll_by(0, 1, 2, h), 0);
}

#[test]
fn opening_history_starts_at_the_newest() {
    let mut state = full_ring();
    state.history_scroll = 5;
    state.set_mode(Mode::Basic);
    state.set_mode(Mode::History);
    assert_eq!(state.history_scroll, 0);
    assert!(MODES.contains(&Mode::History));
}
