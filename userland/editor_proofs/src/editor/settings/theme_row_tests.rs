// NONOS Operating System (AGPL-3.0-or-later)
//! The theme picker, at 1x to 2x: the Appearance card's theme row names the
//! theme in force, System first, and a press on it steps through every named
//! theme and back to System.

use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::paint::PaintBuffer;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{Card, Controls, Row};

use super::card::card_rect;
use super::control_tests::{crop, paint, row_at, H};
use super::pick::{act, Pick};
use super::state::select_nav;
use crate::editor::square_tests::lock;
use crate::editor::strip_tests::W;

/// The Appearance card as it should look with `name` on its theme row.
fn drawn(name: &str) -> Vec<u32> {
    let mut px = vec![roles().bg.argb(); (W * H) as usize];
    let mut fb = PaintBuffer { pixels: &mut px, stride_words: W, width: W, height: H };
    let at = card_rect(W, "Appearance", 1);
    Card::new().title("Appearance").draw(&mut fb, at);
    Row::new("Theme").value(name).chevron().draw(&mut fb, row_at(1, 0));
    crop(&px, at)
}

#[test]
fn the_theme_row_names_the_theme_in_force() {
    let _g = lock();
    let names = ["System", "NONOS Docs", "Light", "Dracula", "Nord"];
    for q in [4, 5, 6, 8] {
        (set_scale(q), select_nav(1));
        let (c, at) = (&mut Controls::acting_on_press(), card_rect(W, "Appearance", 1));
        let first = paint(c);
        for (k, name) in names.iter().enumerate() {
            let shown = crop(&paint(c), at);
            assert!(shown == drawn(name), "choice {k} shows {name} at {q}");
            assert!(names.iter().filter(|n| *n != name).all(|n| shown != drawn(n)));
            assert!(act(Pick::Row(0)), "the theme row acts at {q}");
        }
        assert!(paint(c) == first, "back to System after the last theme at {q}");
    }
    (select_nav(0), set_scale(4));
}
