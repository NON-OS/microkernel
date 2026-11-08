// NONOS Operating System (AGPL-3.0-or-later)
//! The document tab strip's marks: choosing a tab changes only that tab and
//! the one it replaces, a pointer that left or one off the band lights
//! nothing, and a tab with unsaved changes draws the shared Modified mark
//! where a clean one draws Close, a click on either naming that tab's close.

use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::icons::IconId;
use nonos_toolkit::paint::PaintBuffer;
use nonos_toolkit::widgets::rect::contains;
use nonos_toolkit::widgets::{IconButton, Pointer, Tabs};

use super::square_tests::{centre, lock};
use super::strip_tests::{changed, paint, strip, H, NAMES, W};
use crate::editor::strip::Spot;

#[test]
fn choosing_a_tab_marks_only_that_tab_and_the_one_it_leaves() {
    let _g = lock();
    for q in [4, 8] {
        set_scale(q);
        let clean = [false; 3];
        let (s, ptr) = (strip(q, &clean, 0), Pointer::new());
        let diff = changed(&paint(&s, &ptr), &paint(&strip(q, &clean, 2), &ptr));
        let tab_of = |x: i32, y: i32| match s.spot(x, y) {
            Some(Spot::Tab(i) | Spot::Close(i)) => Some(i),
            _ => None,
        };
        assert!(diff.iter().all(|&(x, y)| matches!(tab_of(x, y), Some(0 | 2))), "at {q}");
        for i in [0, 2] {
            assert!(diff.iter().any(|&(x, y)| tab_of(x, y) == Some(i)), "tab {i} at {q}");
        }
    }
    set_scale(4);
}

#[test]
fn a_pointer_that_left_or_is_off_the_band_lights_nothing() {
    let _g = lock();
    for q in [4, 8] {
        set_scale(q);
        let clean = [false; 3];
        let s = strip(q, &clean, 1);
        let unlit = paint(&s, &Pointer::new());
        let (x, y) = centre(s.close(0).unwrap());
        let mut ptr = Pointer::new();
        ptr.moved(x, s.band.y as i32 - 1);
        assert_eq!(paint(&s, &ptr), unlit, "above the band at {q}");
        ptr.moved(x, y);
        ptr.left();
        assert_eq!(paint(&s, &ptr), unlit, "a pointer that left at {q}");
    }
    set_scale(4);
}

#[test]
fn an_unsaved_tab_draws_modified_and_a_clean_one_close_and_both_close_it() {
    let _g = lock();
    for (q, dirty) in [4, 8].into_iter().flat_map(|q| [(q, false), (q, true)]) {
        set_scale(q);
        let flags = [dirty, false, false];
        let s = strip(q, &flags, 1);
        let mark = s.close(0).unwrap();
        assert_eq!(s.spot(centre(mark).0, centre(mark).1), Some(Spot::Close(0)), "at {q}");
        let mut want = vec![0u32; (W * H) as usize];
        let mut fb = PaintBuffer { pixels: &mut want, stride_words: W, width: W, height: H };
        Tabs::new(&NAMES, 1).draw(&mut fb, s.area());
        let icon = if dirty { IconId::Modified } else { IconId::Close };
        IconButton::new(icon).draw(&mut fb, mark);
        let got = paint(&s, &Pointer::new());
        let off = changed(&want, &got).into_iter().filter(|&(x, y)| contains(mark, x, y));
        assert_eq!(off.count(), 0, "mark {icon:?} at {q}");
    }
    set_scale(4);
}
