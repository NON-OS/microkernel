// NONOS Operating System (AGPL-3.0-or-later)
//! The explorer's rows in shared controls, at 1x to 2x: a row lights only
//! where its press acts, a press in its indent acts on it, only the selected
//! node is marked, and a pointer that left lights nothing.

use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::paint::PaintBuffer;
use nonos_toolkit::widgets::{Controls, Rect, Step};

use super::square_tests::lock;
use super::strip_tests::{changed, W};
use crate::editor::explorer::{Entry, Explorer};

const H: u32 = 240;
const fn entry(name: &'static str, depth: u16, dir: bool) -> Entry<'static> {
    Entry { name, depth, dir, open: dir }
}

const TREE: [Entry; 5] = [
    entry("src", 0, true),
    entry("editor", 1, true),
    entry("a-much-longer-file-name.rs", 2, false),
    entry("main.rs", 1, false),
    entry("notes.txt", 0, false),
];

fn paint(q: u32, selected: usize, c: &mut Controls<usize>) -> Vec<u32> {
    let band = Rect { x: 46 * q / 4, y: 2 * q, w: 220 * q / 4, h: 24 * q };
    let ex = Explorer { entries: &TREE, first: 1, selected, band, row_h: 24 * q / 4 };
    let mut px = vec![0u32; (W * H) as usize];
    let mut fb = PaintBuffer { pixels: &mut px, stride_words: W, width: W, height: H };
    ex.paint(&mut fb, c);
    px
}

#[test]
fn a_row_lights_and_presses_only_where_its_press_acts() {
    let _g = lock();
    for (q, vi) in [4, 5, 6, 8].into_iter().flat_map(|q| (1..TREE.len()).map(move |vi| (q, vi))) {
        set_scale(q);
        let mut c = Controls::acting_on_press();
        let unlit = paint(q, 0, &mut c);
        let (x, y) = (50 * q as i32, (5 + 6 * vi as i32 - 6) * q as i32);
        assert_eq!(c.pressed(12 * q as i32, y), Step::Act(vi), "indent of {vi} at {q}");
        (c.released(), c.moved(x, y));
        let lit = paint(q, 0, &mut c);
        c.pressed(x, y);
        let pressed = paint(q, 0, &mut c);
        for diff in [changed(&unlit, &lit), changed(&unlit, &pressed)] {
            let here = diff.iter().all(|&(x, y)| c.at(x, y) == Some(vi));
            assert!(!diff.is_empty() && here, "row {vi} at {q}");
        }
        (c.released(), c.left());
        assert_eq!(paint(q, 0, &mut c), unlit, "a pointer that left, row {vi} at {q}");
    }
    set_scale(4);
}

#[test]
fn only_the_selected_node_is_marked() {
    let _g = lock();
    for q in [4, 5, 6, 8] {
        set_scale(q);
        let c = &mut Controls::acting_on_press();
        let none = paint(q, 9, c);
        assert_eq!(paint(q, 0, c), none, "an unshown node marks nothing at {q}");
        for sel in 1..TREE.len() {
            let diff = changed(&none, &paint(q, sel, c));
            let on = |&(x, y): &(i32, i32)| c.at(x, y) == Some(sel);
            assert!(!diff.is_empty() && diff.iter().all(on), "{sel} at {q}");
        }
    }
    set_scale(4);
}
