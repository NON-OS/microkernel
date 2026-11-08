// NONOS Operating System (AGPL-3.0-or-later)
//! The activity bar as navigation items in shared controls, at 1x and 2x: a
//! hovered or pressed row lights only where its press acts, only the screen
//! on show is marked, and a pointer that left or is off the rows lights nothing.

use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::paint::PaintBuffer;
use nonos_toolkit::widgets::{Controls, Rect, Step};

use super::square_tests::{centre, lock};
use super::strip_tests::changed;
use crate::editor::activity_bar::{paint_rail, row, ROWS};
use crate::editor::screen::Screen;

const H: u32 = 320;

fn band(q: u32) -> Rect {
    Rect { x: 0, y: 3 * q, w: 46 * q / 4, h: 132 * q / 4 }
}

fn paint(q: u32, on_show: (Screen, bool), c: &mut Controls<Screen>) -> Vec<u32> {
    let w = super::strip_tests::W;
    let mut px = vec![0u32; (w * H) as usize];
    let mut fb = PaintBuffer { pixels: &mut px, stride_words: w, width: w, height: H };
    paint_rail(&mut fb, Rect { x: 0, y: 0, w: 46 * q / 4, h: H }, band(q), on_show, c);
    px
}

#[test]
fn a_row_lights_and_presses_only_where_its_press_acts() {
    let _g = lock();
    for (q, i) in [4, 8].into_iter().flat_map(|q| (0..ROWS.len()).map(move |i| (q, i))) {
        set_scale(q);
        let (target, show) = (ROWS[i].0, (Screen::Editor, false));
        let mut c = Controls::acting_on_press();
        let unlit = paint(q, show, &mut c);
        let (x, y) = centre(row(band(q), i));
        c.moved(x, y);
        let lit = paint(q, show, &mut c);
        assert!(c.pressed(x, y) == Step::Act(target), "row {i} at {q}");
        let pressed = paint(q, show, &mut c);
        for diff in [changed(&unlit, &lit), changed(&unlit, &pressed)] {
            let here = diff.iter().all(|&(x, y)| c.at(x, y) == Some(target));
            assert!(!diff.is_empty() && here, "row {i} at {q}");
        }
        c.released();
        c.left();
        assert_eq!(paint(q, show, &mut c), unlit, "a pointer that left, row {i} at {q}");
        c.moved(x, band(q).y as i32 - 1);
        assert_eq!(paint(q, show, &mut c), unlit, "above the rows at {q}");
    }
    set_scale(4);
}

#[test]
fn only_the_screen_on_show_is_marked() {
    let _g = lock();
    for q in [4, 8] {
        set_scale(q);
        let c = &mut Controls::acting_on_press();
        let home = paint(q, (Screen::Home, true), c);
        let settings = paint(q, (Screen::Settings, true), c);
        let shut = paint(q, (Screen::Editor, false), c);
        let open = paint(q, (Screen::Editor, true), c);
        let only = |diff: Vec<(i32, i32)>, rows: &[Screen]| {
            let on = |&(x, y): &(i32, i32)| c.at(x, y).is_some_and(|a| rows.contains(&a));
            !diff.is_empty() && diff.iter().all(on)
        };
        assert!(only(changed(&home, &settings), &[Screen::Home, Screen::Settings]), "at {q}");
        assert!(only(changed(&home, &shut), &[Screen::Home]), "explorer shut at {q}");
        assert!(only(changed(&shut, &open), &[Screen::Editor]), "explorer open at {q}");
    }
    set_scale(4);
}
