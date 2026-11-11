// NONOS Operating System (AGPL-3.0-or-later)
//! The ribbon keeps logical metrics and draws them at the display scale: at
//! 1x to 2x every length is its 1x length through `scale().px`, each pill is
//! as wide as its label measured at the chrome size, the cells sit in their
//! band, and a window grown by the same scale holds the whole row.

use nonos_app_skeleton::measure_ttf;
use nonos_toolkit::design::scale::{scale, set_scale};

use super::cells::cells;
use super::hit::{ribbon_hit, RibbonHit};
use super::items::{RibbonItem, ICON_COUNT, ICON_TABLE, TOGGLES};
use super::metrics::{band_top, cell_h, cell_top, lengths, RibbonCell};
use super::snapshot::ribbon_state;
use crate::editor::manifest::{HEIGHT, WIDTH};
use crate::editor::menu_view::bounds;
use crate::editor::square_tests::lock;
use crate::layout::{activity_w, chrome_px, ribbon_h, tabbar_h, titlebar_h};
use crate::state::State;

const LABELS: [&str; 3] = ["Heading 2", "JetBrains Mono", "48"];

/// The item a press at (`x`, `y`) lands on, with no pill open.
fn hit_at(row: &[RibbonCell], x: u32, y: u32) -> Option<RibbonItem> {
    let (w, h, st) = (scale().px(WIDTH), scale().px(HEIGHT), ribbon_state(&State::new()));
    match ribbon_hit(row, None, &st, bounds(w, h), x as i32, y as i32) {
        RibbonHit::Cell(item) => Some(item),
        _ => None,
    }
}

fn sizes() -> Vec<u32> {
    let l = lengths();
    vec![l.bar_pad, l.pill_pad, l.chevron_w, l.chevron, l.gap, l.sep_gap, l.square_w]
}

#[test]
fn every_ribbon_size_grows_with_the_scale_and_the_row_still_fits() {
    let _g = lock();
    set_scale(4);
    let base = sizes();
    for q in [4, 5, 6, 8] {
        set_scale(q);
        let (s, l) = (scale(), lengths());
        let want: Vec<u32> = base.iter().map(|&n| s.px(n)).collect();
        assert_eq!(sizes(), want, "q={q}: a length is not its 1x length at the scale");
        assert_eq!(band_top(), titlebar_h() + tabbar_h(), "q={q}: the band is not under the tabs");
        assert!(cell_top() >= band_top(), "q={q}: a cell rises out of its band");
        assert!(cell_top() + cell_h() <= band_top() + ribbon_h(), "q={q}: a cell falls out");
        let row = cells(&LABELS);
        assert_eq!(row.len(), LABELS.len() + TOGGLES.len() + ICON_COUNT);
        assert_eq!(row[0].x0, activity_w() + l.bar_pad, "q={q}: the row does not start at its pad");
        for (c, next) in row.iter().zip(row.iter().skip(1)) {
            assert!(c.x1 + l.gap <= next.x0, "q={q}: two cells overlap");
        }
        for c in &row {
            let w = match c.item {
                RibbonItem::Pill(p) => {
                    measure_ttf(LABELS[p], chrome_px()) as u32 + l.pill_pad * 2 + l.chevron_w
                }
                _ => l.square_w,
            };
            assert_eq!(c.x1 - c.x0, w, "q={q}: a cell is not its measured width");
            let (top, bottom) = (band_top(), band_top() + ribbon_h() - 1);
            for (x, y) in [(c.x0, top), (c.x1 - 1, bottom), ((c.x0 + c.x1) / 2, cell_top())] {
                assert!(hit_at(&row, x, y) == Some(c.item), "q={q}: a cell misses at ({x}, {y})");
            }
        }
        let table = row[row.len() - ICON_COUNT + ICON_TABLE].item;
        assert!(table == RibbonItem::Icon(ICON_TABLE), "q={q}: the table square is not laid out");
        assert!(row.last().unwrap().x1 <= s.px(WIDTH), "q={q}: the row leaves the window");
    }
    set_scale(4);
}
