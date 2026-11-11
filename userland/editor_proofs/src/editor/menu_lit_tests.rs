// NONOS Operating System (AGPL-3.0-or-later)
//! The editor's menus drawn as the shared menu: at 1x to 2x, under a title,
//! a ribbon pill and a press near the window's edge, every pixel the hovered
//! row lights lies where a click on that row lands, and a pointer off the
//! menu, or one that has left, lights nothing.

use std::sync::{Mutex, MutexGuard, PoisonError};

use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::paint::PaintBuffer;
use nonos_toolkit::widgets::{Item, Menu, Pointer, Rect};

use crate::editor::menu_view::{bounds, draw, hit, lit, place, ticked};
use crate::editor::menubar::items::{items, TITLES};

const W: u32 = 900;
const H: u32 = 900;
pub(super) static GLOBALS: Mutex<()> = Mutex::new(());

const ANCHORS: [Rect; 3] = [
    Rect { x: 8, y: 0, w: 60, h: 26 },
    Rect { x: 120, y: 58, w: 140, h: 40 },
    Rect { x: 880, y: 880, w: 0, h: 0 },
];

fn menus() -> Vec<Vec<Item<'static>>> {
    let mut all: Vec<Vec<Item>> = (0..TITLES.len()).map(items).collect();
    let sizes: &'static [String] = ["10", "16", "48"].map(String::from).to_vec().leak();
    all.push(ticked(sizes, "16"));
    all
}

fn paint(rows: &[Item], at: Rect, ptr: &Pointer) -> Vec<u32> {
    let mut px = vec![0u32; (W * H) as usize];
    let mut fb = PaintBuffer { pixels: &mut px, stride_words: W, width: W, height: H };
    draw(&mut fb, rows, at, ptr);
    px
}

fn mid(rows: &[Item], at: Rect, i: usize) -> (i32, i32) {
    let x = (at.x + at.w / 2) as i32;
    let on = |y: &i32| hit(rows, at, x, *y) == Some(i);
    let ys: Vec<i32> = (at.y as i32..(at.y + at.h) as i32).filter(on).collect();
    (x, (ys[0] + ys[ys.len() - 1]) / 2)
}

#[test]
fn every_lit_pixel_is_where_the_click_lands() {
    let _g: MutexGuard<()> = GLOBALS.lock().unwrap_or_else(PoisonError::into_inner);
    for q in [4, 5, 6, 8] {
        set_scale(q);
        for (rows, anchor) in menus().iter().flat_map(|m| ANCHORS.map(|a| (m, a))) {
            let at = place(rows, anchor, bounds(W, H));
            let unlit = paint(rows, at, &Pointer::new());
            for i in 0..Menu::new(rows).shown(at) {
                let (x, y) = mid(rows, at, i);
                let mut ptr = Pointer::new();
                ptr.moved(x, y);
                assert_eq!(lit(rows, at, &ptr), Some(i), "row {i} at {q}");
                let lit_px = paint(rows, at, &ptr);
                let diff = (0..unlit.len()).filter(|&n| unlit[n] != lit_px[n]);
                let at_px = |n: usize| ((n as u32 % W) as i32, (n as u32 / W) as i32);
                let hits: Vec<_> =
                    diff.map(|n| at_px(n)).map(|(x, y)| hit(rows, at, x, y)).collect();
                assert!(!hits.is_empty() && hits.iter().all(|&r| r == Some(i)), "{i} at {q}");
                ptr.moved(at.x as i32 - 1, y);
                assert_eq!(lit(rows, at, &ptr), None);
                ptr.left();
                assert_eq!(paint(rows, at, &ptr), unlit, "a pointer that left lights nothing");
            }
        }
    }
    set_scale(4);
}
