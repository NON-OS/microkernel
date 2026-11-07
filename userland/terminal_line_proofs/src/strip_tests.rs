// NONOS Operating System (AGPL-3.0-or-later)
//! The titlebar strip: every pixel a control lights when the pointer is on
//! it lies where `spot` names that control, at 1x and 2x, with one tab and
//! with as many as the window takes.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::design::scale::set_scale;

use crate::strip_boxes::{drawn, serial, SETS, SIZES};
use crate::strip_ptr::StripPointer;
use crate::strip_spot::{spot, Spot};
use crate::tab_bar::draw_tab_bar;

fn paint(names: &[&str], sp: &StripPointer) -> Vec<u32> {
    let (w, h) = sp.size;
    let mut px = vec![0u32; (w * h) as usize];
    let mut fb = PaintBuffer { pixels: &mut px, stride_words: w, width: w, height: h };
    draw_tab_bar(names, 0, sp, &mut fb);
    px
}

fn same_tab(a: Spot, b: Option<Spot>) -> bool {
    match (a, b) {
        (Spot::Tab(i) | Spot::Close(i), Some(Spot::Tab(j) | Spot::Close(j))) => i == j,
        (a, b) => Some(a) == b,
    }
}

#[test]
fn every_lit_pixel_is_where_the_click_lands() {
    let _g = serial();
    for (q, names, size) in
        [4, 8].into_iter().flat_map(|q| SETS.map(|n| SIZES.map(|s| (q, n, s)))).flatten()
    {
        set_scale(q);
        let plain = StripPointer { size, ..Default::default() };
        let base = paint(names, &plain);
        for (want, r) in drawn(names, 0, size) {
            let mut sp = plain;
            sp.ptr.pressed((r.x + r.w / 2) as i32, (r.y + r.h / 2) as i32);
            let lit = paint(names, &sp);
            assert!(
                lit != base || matches!(want, Spot::Tab(0)),
                "{want:?} lights nothing at {q} {size:?}"
            );
            for (n, _) in lit.iter().zip(&base).enumerate().filter(|(_, (a, b))| a != b) {
                let (x, y) = ((n as u32 % size.0) as i32, (n as u32 / size.0) as i32);
                let got = spot(names, 0, size, x, y);
                assert!(same_tab(want, got), "{want:?} {r:?} {q} {size:?} {names:?} lit ({x}, {y}), a click there is {got:?}");
            }
        }
    }
    set_scale(4);
}
