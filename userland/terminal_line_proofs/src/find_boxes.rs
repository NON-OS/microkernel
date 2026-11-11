// NONOS Operating System (AGPL-3.0-or-later)
//! The search bar's test bench: a search in each state, the bar painted over
//! a prompt row with the rail open and shut, and the pixels two frames differ in.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::widgets::Rect;
use nonos_vt::Pos;

use crate::find_bar::draw_find_bar;
use crate::footer::footer_h;
use crate::select::Find;

pub(crate) const W: u32 = 1000;
pub(crate) const H: u32 = 400;
pub(crate) const ROWS: [(u32, u32, u32); 3] = [(0, 1000, 22), (320, 680, 22), (320, 680, 44)];

pub(crate) fn find(query: &str, hit: bool, case: bool) -> Find {
    let at = Pos { line: 3, col: 1 };
    Find { query: query.into(), hit: hit.then_some((at, at)), case }
}

pub(crate) fn paint(f: &Find, (input, footer): (Rect, Rect)) -> Vec<u32> {
    let mut px = vec![0u32; (W * H) as usize];
    let mut fb = PaintBuffer { pixels: &mut px, stride_words: W, width: W, height: H };
    draw_find_bar(f, &mut fb, input, footer);
    px
}

pub(crate) fn changed(a: &[u32], b: &[u32]) -> Vec<(i32, i32)> {
    let at = |n: usize| ((n as u32 % W) as i32, (n as u32 / W) as i32);
    a.iter().zip(b).enumerate().filter(|(_, (p, q))| p != q).map(|(n, _)| at(n)).collect()
}

pub(crate) fn each(mut check: impl FnMut((Rect, Rect))) {
    for q in [4, 5, 6, 8] {
        set_scale(q);
        for (x, w, h) in ROWS {
            let footer = Rect { x: 0, y: 300 + h, w: W, h: footer_h() };
            check((Rect { x, y: 300, w, h }, footer));
        }
    }
}
