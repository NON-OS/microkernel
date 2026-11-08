// NONOS Operating System (AGPL-3.0-or-later)
//! The ribbon's square controls drawn as the shared button: at 1x and 2x,
//! every pixel a hover or a press changes lies in the column a click on that
//! square lands in, and a pointer off the column lights nothing.

use std::sync::{MutexGuard, PoisonError};

use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::icons::IconId;
use nonos_toolkit::paint::PaintBuffer;
use nonos_toolkit::widgets::rect::contains;
use nonos_toolkit::widgets::{Pointer, Rect};

use super::menu_lit_tests::GLOBALS;
use crate::editor::square::Square;

const W: u32 = 700;
const H: u32 = 240;

pub(super) fn row(q: u32, on: bool, live: bool) -> Vec<Square> {
    let (d, icon) = (|n: u32| n * q / 4, IconId::TextBold);
    let at = |i: u32| Rect { x: d(100 + 36 * i), y: d(58), w: d(30), h: d(40) };
    let sq =
        |b: Rect| Square { band: b, body: Rect { y: b.y + d(6), h: d(28), ..b }, icon, on, live };
    (0..4).map(|i| sq(at(i))).collect()
}

pub(super) fn paint(squares: &[Square], ptr: &Pointer) -> Vec<u32> {
    let mut px = vec![0u32; (W * H) as usize];
    let mut fb = PaintBuffer { pixels: &mut px, stride_words: W, width: W, height: H };
    squares.iter().for_each(|s| s.draw(&mut fb, ptr));
    px
}

pub(super) fn centre(r: Rect) -> (i32, i32) {
    ((r.x + r.w / 2) as i32, (r.y + r.h / 2) as i32)
}

pub(super) fn lock() -> MutexGuard<'static, ()> {
    GLOBALS.lock().unwrap_or_else(PoisonError::into_inner)
}

#[test]
fn every_lit_or_pressed_pixel_is_where_the_click_lands() {
    let _g = lock();
    for (q, on) in [4, 8].into_iter().flat_map(|q| [(q, false), (q, true)]) {
        set_scale(q);
        let squares = row(q, on, true);
        let unlit = paint(&squares, &Pointer::new());
        for (i, s) in squares.iter().enumerate() {
            let (x, y) = centre(s.band);
            let (mut hover, mut press) = (Pointer::new(), Pointer::new());
            (hover.moved(x, y), press.pressed(x, y));
            for px in [paint(&squares, &hover), paint(&squares, &press)] {
                let at = |n: usize| ((n as u32 % W) as i32, (n as u32 / W) as i32);
                let diff: Vec<_> = (0..px.len()).filter(|&n| px[n] != unlit[n]).map(at).collect();
                assert!(!diff.is_empty(), "square {i} at {q} shows nothing");
                assert!(diff.iter().all(|&(x, y)| contains(s.band, x, y)), "{i} at {q}");
            }
            hover.moved(s.band.x as i32 - 1, y);
            assert_eq!(paint(&squares, &hover), unlit, "off the column lights nothing");
        }
    }
    set_scale(4);
}
