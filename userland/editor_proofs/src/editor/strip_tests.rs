// NONOS Operating System (AGPL-3.0-or-later)
//! The tab strip as the shared tabs, at 1x and 2x: a hovered or pressed control lights only
//! where its click lands (the chosen tab shows none); a close mark over its tab, only its box.

use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::paint::PaintBuffer;
use nonos_toolkit::widgets::rect::contains;
use nonos_toolkit::widgets::{Pointer, Rect};

use super::square_tests::{centre, lock};
use crate::editor::strip::{Spot, Strip, TOOLS};

pub(super) const W: u32 = 1400;
pub(super) const H: u32 = 160;
pub(super) const NAMES: [&str; 3] = ["main.rs", "notes.txt", "a-much-longer-name.md"];

pub(super) fn strip<'a>(q: u32, dirty: &'a [bool], active: usize) -> Strip<'a> {
    let d = |n: u32| n * q / 4;
    Strip { names: &NAMES, dirty, active, band: Rect { x: d(60), y: d(26), w: d(600), h: d(32) } }
}

pub(super) fn paint(s: &Strip, ptr: &Pointer) -> Vec<u32> {
    let mut px = vec![0u32; (W * H) as usize];
    let mut fb = PaintBuffer { pixels: &mut px, stride_words: W, width: W, height: H };
    s.draw(&mut fb, ptr);
    px
}

pub(super) fn changed(a: &[u32], b: &[u32]) -> Vec<(i32, i32)> {
    let at = |n: usize| ((n as u32 % W) as i32, (n as u32 / W) as i32);
    (0..a.len()).filter(|&n| a[n] != b[n]).map(at).collect()
}

fn same_control(a: Spot, b: Option<Spot>) -> bool {
    match (a, b) {
        (Spot::Tab(i) | Spot::Close(i), Some(Spot::Tab(j) | Spot::Close(j))) => i == j,
        (a, b) => Some(a) == b,
    }
}

#[test]
fn every_lit_or_pressed_pixel_is_where_the_click_lands() {
    let _g = lock();
    for (q, dirty) in [4, 8].into_iter().flat_map(|q| [(q, false), (q, true)]) {
        set_scale(q);
        let flags = [dirty; 3];
        let s = strip(q, &flags, 1);
        let unlit = paint(&s, &Pointer::new());
        let tabs = (0..3).flat_map(|i| [(Spot::Tab(i), s.tab(i)), (Spot::Close(i), s.close(i))]);
        let tools = (0..TOOLS.len()).map(|t| (Spot::Tool(t), Some(s.tools()[t])));
        for (spot, r) in tabs.chain(tools).map(|(p, r)| (p, r.expect("a box"))) {
            let (x, y) = match spot {
                Spot::Tab(_) => (r.x as i32 + 4, centre(r).1),
                _ => centre(r),
            };
            assert_eq!(s.spot(x, y), Some(spot), "{spot:?} at {q}");
            let (mut hover, mut press) = (Pointer::new(), Pointer::new());
            (hover.moved(x, y), press.pressed(x, y));
            for px in [paint(&s, &hover), paint(&s, &press)] {
                let diff = changed(&unlit, &px);
                assert!(!diff.is_empty() || spot == Spot::Tab(1), "{spot:?} at {q} is unlit");
                let ours = diff.iter().all(|&(x, y)| same_control(spot, s.spot(x, y)));
                assert!(ours, "{spot:?} at {q}");
            }
            if let Spot::Close(i) = spot {
                let mut on_tab = Pointer::new();
                on_tab.moved(s.tab(i).unwrap().x as i32 + 4, y);
                let diff = changed(&paint(&s, &on_tab), &paint(&s, &hover));
                let inside = diff.iter().all(|&(x, y)| contains(r, x, y));
                assert!(!diff.is_empty() && inside, "close {i} at {q}");
            }
        }
    }
    set_scale(4);
}
