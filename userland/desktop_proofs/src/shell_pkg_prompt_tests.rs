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

//! The install prompt draws every capability bit, named as it names them,
//! uncut with no warning where `DIALOG_LINES` cuts it; a short screen warns.

use nonos_toolkit::icons::IconId;
use nonos_toolkit::paint::measure_ttf;
use nonos_toolkit::widgets::{Dialog, Rect, Surface};

use super::cap_names::append;
use super::whole_prompt::whole;
use crate::shell_menu_view::bounds;
use crate::shell_prompt_buttons::APPROVE;

/// The prompt's warning, as `pkg_consent` words it.
const NOTE: &str = "! caps hidden - do not approve";

/// A surface that keeps the text drawn on it and paints nothing.
struct Texts(Vec<String>);
impl Surface for Texts {
    fn fill_rect(&mut self, _: u32, _: u32, _: u32, _: u32, _: u32) {}
    fn blend_rect(&mut self, _: u32, _: u32, _: u32, _: u32, _: u32) {}
    fn fill_round(&mut self, _: u32, _: u32, _: u32, _: u32, _: u32, _: u32) {}
    fn stroke_round(&mut self, _: u32, _: u32, _: u32, _: u32, _: u32, _: u32, _: u32) {}
    fn circle(&mut self, _: u32, _: u32, _: u32, _: u32) {}
    fn text_ttf(&mut self, x: i32, _: i32, text: &str, _: u32, px: f32) -> i32 {
        self.0.push(text.to_string());
        x + measure_ttf(text, px)
    }
    fn icon(&mut self, _: IconId, _: u32, _: u32, _: u32, _: u32) {}
}

/// The body `pkg_consent` words for a package granted every bit.
fn body() -> String {
    let mut caps = Vec::new();
    append(u64::MAX, &mut caps);
    let tail = String::from_utf8(caps).unwrap();
    format!("gui_demo from com.example.gui_demo, Publisher-signed. Caps: {tail}")
}

/// The body's lines the prompt draws in `at`, and whether it warns.
fn drawn(body: &str, at: Rect) -> (Vec<String>, bool) {
    let mut texts = Texts(Vec::new());
    let d = Dialog::new("Install package?").body(body).buttons(&APPROVE);
    whole(d, NOTE, at).draw(&mut texts, at);
    let (warns, all) = (texts.0.iter().any(|t| t == NOTE), texts.0.len());
    let n = all - APPROVE.len() - usize::from(warns);
    (texts.0[1..n].to_vec(), warns)
}

pub(crate) fn pkg_prompt_shows_the_whole_list(tag: &str, w: u32, h: u32) {
    let (body, screen) = (body(), bounds(w, h));
    let (lines, warns) = drawn(&body, screen);
    assert!(!warns && lines.join(" ") == body, "{tag}: {lines:?}");
    let five = Dialog::new("Install package?").body(&body).buttons(&APPROVE);
    assert!(five.cut(screen), "{tag}: the list fits `DIALOG_LINES`");
    let short = bounds(w, five.body_lines(u32::MAX).size().1 * 3 / 4);
    let (lines, warns) = drawn(&body, short);
    let cut = lines.last().is_some_and(|l| l.ends_with('…'));
    assert!(warns && cut, "{tag}: a short screen warns not over {lines:?}");
}
