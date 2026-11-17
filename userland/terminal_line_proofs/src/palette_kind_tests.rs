// NONOS Operating System (AGPL-3.0-or-later)
//! The palette's rows say what kind they are: at 1x and 2x, over a wide and
//! a narrow body and after every query, each row drawn leads with its
//! entry's tag, `cmd`, `run`, `tab`, `dir` or `act`, before its label.

use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::icons::IconId;
use nonos_toolkit::paint::measure_ttf;
use nonos_toolkit::widgets::Surface;

use crate::palette::Kind;
use crate::palette_boxes::{cases, entries};
use crate::palette_view::{items, matches, view};
use crate::strip_boxes::serial;

/// A surface that keeps each run of text with its line top, painting nothing.
struct Runs(Vec<(i32, String)>);

impl Surface for Runs {
    fn fill_rect(&mut self, _: u32, _: u32, _: u32, _: u32, _: u32) {}
    fn blend_rect(&mut self, _: u32, _: u32, _: u32, _: u32, _: u32) {}
    fn fill_round(&mut self, _: u32, _: u32, _: u32, _: u32, _: u32, _: u32) {}
    fn stroke_round(&mut self, _: u32, _: u32, _: u32, _: u32, _: u32, _: u32, _: u32) {}
    fn circle(&mut self, _: u32, _: u32, _: u32, _: u32) {}
    fn text_ttf(&mut self, x: i32, top: i32, text: &str, _: u32, px: f32) -> i32 {
        self.0.push((top, text.to_string()));
        x + measure_ttf(text, px)
    }
    fn icon(&mut self, _: IconId, _: u32, _: u32, _: u32, _: u32) {}
}

/// The tag the palette has always shown for `kind`.
fn spelled(kind: Kind) -> &'static str {
    match kind {
        Kind::Command => "cmd",
        Kind::History => "run",
        Kind::Session => "tab",
        Kind::Project => "dir",
        Kind::Action => "act",
    }
}

#[test]
fn every_row_leads_with_its_entry_kind() {
    let _g = serial();
    let all = entries();
    for (q, body, needle) in cases() {
        set_scale(q);
        let m = matches(&all, needle.as_bytes());
        let rows = items(&m);
        let v = view(body, &rows);
        let mut runs = Runs(Vec::new());
        v.menu.draw(&mut runs, v.at);
        let mut firsts: Vec<(i32, String)> = Vec::new();
        for (top, text) in runs.0 {
            if firsts.last().is_none_or(|(t, _)| *t != top) {
                firsts.push((top, text));
            }
        }
        let want: Vec<&str> = m[..v.rows].iter().map(|e| spelled(e.kind)).collect();
        let got: Vec<&str> = firsts.iter().map(|(_, t)| t.as_str()).collect();
        assert_eq!(got, want, "{needle:?} in {body:?} at {q}/4");
    }
    set_scale(4);
}
