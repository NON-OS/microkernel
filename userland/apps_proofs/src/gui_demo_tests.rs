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

//! GUI Demo, the reference interactive app: a click lands on the control its
//! paint drew there, Reset is unavailable until there is something to reset,
//! and Esc closes.

use alloc::vec;

use nonos_app_skeleton::InputKind::{ButtonDown, ButtonUp, KeyDown};
use nonos_app_skeleton::{App, EventOutcome, InputEvent, PaintBuffer, KEY_ESC};

use crate::feed_tests::ev;
use crate::gui_demo::action::Action;
use crate::gui_demo::app::GuiDemo;

fn painted(d: &mut GuiDemo) {
    let (width, height) = (380, 190);
    let mut pixels = vec![0u32; (width * height) as usize];
    let mut fb = PaintBuffer { pixels: &mut pixels, stride_words: width, width, height };
    d.paint(&mut fb);
}

fn click(d: &mut GuiDemo, a: Action) -> EventOutcome {
    let (x, y) = (0..190)
        .flat_map(|y| (0..380).map(move |x| (x, y)))
        .find(|&(x, y)| d.controls.at(x, y) == Some(a))
        .unwrap_or_else(|| panic!("{a:?} is drawn nowhere live"));
    d.on_event(ev(ButtonDown, x, y));
    let out = d.on_event(ev(ButtonUp, x, y));
    painted(d);
    out
}

#[test]
fn each_control_acts_where_it_is_drawn() {
    let mut d = GuiDemo::new();
    painted(&mut d);
    assert!(click(&mut d, Action::Count) == EventOutcome::Repaint);
    assert_eq!((d.clicks, d.last), (1, Some(Action::Count)));
    click(&mut d, Action::Toggle);
    click(&mut d, Action::Remember);
    assert!(d.on && d.remember);
    click(&mut d, Action::Reset);
    assert_eq!((d.clicks, d.last), (0, Some(Action::Reset)));
}

#[test]
fn reset_is_unavailable_while_nothing_is_counted() {
    let mut d = GuiDemo::new();
    painted(&mut d);
    assert!(d.controls.state(Action::Reset).disabled);
    let any = (0..190).any(|y| (0..380).any(|x| d.controls.at(x, y) == Some(Action::Reset)));
    assert!(!any);
}

#[test]
fn esc_closes() {
    let mut d = GuiDemo::new();
    let esc = InputEvent { code: KEY_ESC, ..ev(KeyDown, 0, 0) };
    assert!(d.on_event(esc) == EventOutcome::Close);
}
