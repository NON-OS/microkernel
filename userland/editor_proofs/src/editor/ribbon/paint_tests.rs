// NONOS Operating System (AGPL-3.0-or-later)
//! The ribbon's painter at 1x to 2x: it paints only inside its band, records
//! the very cells the walk lays out, so a press lands on the cell drawn under
//! it, and an open pill's menu takes a press on the row drawn there.

use nonos_toolkit::design::scale::{scale, set_scale};
use nonos_toolkit::paint::PaintBuffer;
use nonos_toolkit::widgets::Pointer;

use super::cells::cells;
use super::hit::{ribbon_hit, RibbonHit};
use super::items::pill_labels;
use super::metrics::band_top;
use super::paint::paint_ribbon;
use super::panel::{panel_at, picked};
use super::snapshot::ribbon_state;
use crate::editor::manifest::{HEIGHT, WIDTH};
use crate::editor::menu_view::{bounds, hit, ticked};
use crate::editor::square_tests::lock;
use crate::layout::ribbon_h;
use crate::state::State;

#[test]
fn the_ribbon_paints_its_band_and_records_the_cells_it_drew() {
    let _g = lock();
    let doc = State::new();
    for q in [4, 5, 6, 8] {
        set_scale(q);
        let (w, h, st) = (scale().px(WIDTH), scale().px(HEIGHT), ribbon_state(&doc));
        let mut px = vec![0u32; (w * h) as usize];
        let mut fb = PaintBuffer { pixels: &mut px, stride_words: w, width: w, height: h };
        let drawn = paint_ribbon(&mut fb, w, &st, None, &Pointer::new());
        let labels = [st.style.as_str(), st.font.as_str(), st.size.as_str()];
        let walked = cells(&labels);
        let spans = |c: &[super::metrics::RibbonCell]| -> Vec<(u32, u32)> {
            c.iter().map(|c| (c.x0, c.x1)).collect()
        };
        assert_eq!(spans(&drawn), spans(&walked), "q={q}: the painter drew other cells");
        let band = band_top()..band_top() + ribbon_h();
        let stray = (0..px.len()).find(|&n| px[n] != 0 && !band.contains(&(n as u32 / w)));
        assert_eq!(stray, None, "q={q}: the ribbon painted outside its band");
        for pill in 0..labels.len() {
            let names = pill_labels(pill);
            let rows = ticked(&names, picked(&st, pill));
            let at = panel_at(&drawn, pill, &rows, bounds(w, h));
            let (x, y) = ((at.x + at.w / 2) as i32, (at.y + at.h / 2) as i32);
            let got = match ribbon_hit(&drawn, Some(pill), &st, bounds(w, h), x, y) {
                RibbonHit::Row(r) => Some(r),
                _ => None,
            };
            assert_eq!(got, hit(&rows, at, x, y), "q={q}: pill {pill}'s menu misses its row");
            assert!(got.is_some(), "q={q}: pill {pill}'s menu has no row at its middle");
        }
    }
    set_scale(4);
}
