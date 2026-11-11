// NONOS Operating System (AGPL-3.0-or-later)
//! The menu bar keeps logical metrics and draws them at the display scale: at
//! 1x to 2x its pads are their 1x length through `scale().px`, each title is
//! as wide as its label measured at the chrome size, the window grown by the
//! same scale holds them all, and a press lands on a title across exactly the
//! span the painter fills for it.

use nonos_app_skeleton::measure_ttf;
use nonos_toolkit::design::scale::{scale, set_scale};

use super::hit::{menubar_hit, MenuHit};
use super::items::{items, TITLES};
use super::metrics::{bar_x0, panel_at, text_top, title_pad, title_spans};
use crate::editor::manifest::{HEIGHT, WIDTH};
use crate::editor::menu_view::{bounds, hit};
use crate::editor::square_tests::lock;
use crate::layout::{chrome_px, titlebar_h};

fn press(open: Option<usize>, x: u32, y: u32) -> MenuHit {
    let (w, h) = (scale().px(WIDTH), scale().px(HEIGHT));
    menubar_hit(&title_spans(), open, bounds(w, h), x as i32, y as i32)
}

fn title_at(x: u32, y: u32) -> Option<usize> {
    match press(None, x, y) {
        MenuHit::Title(i) => Some(i),
        _ => None,
    }
}

#[test]
fn every_title_grows_with_the_scale_and_is_hit_where_it_is_drawn() {
    let _g = lock();
    set_scale(4);
    let base = [bar_x0(), title_pad()];
    for q in [4, 5, 6, 8] {
        set_scale(q);
        let s = scale();
        assert_eq!([bar_x0(), title_pad()], base.map(|n| s.px(n)), "q={q}: a pad is not scaled");
        let (spans, band) = (title_spans(), titlebar_h() - s.hair());
        assert_eq!(spans[0].x0, bar_x0(), "q={q}: the first title is not at its pad");
        assert!(spans.last().unwrap().x1 <= s.px(WIDTH), "q={q}: a title leaves the window");
        assert!(text_top(band) + 1 < band, "q={q}: the label has no room in the bar");
        for (i, span) in spans.iter().enumerate() {
            let w = measure_ttf(TITLES[i], chrome_px()) as u32 + title_pad() * 2;
            assert_eq!(span.x1 - span.x0, w, "q={q}: title {i} is not its measured width");
            for (x, y) in [(span.x0, 0), (span.x1 - 1, band - 1), ((span.x0 + span.x1) / 2, 1)] {
                assert_eq!(title_at(x, y), Some(i), "q={q}: title {i} misses at ({x}, {y})");
            }
            assert_eq!(title_at(span.x0, titlebar_h()), None, "q={q}: title {i} hits below");
            let (rows, all) = (items(i), bounds(s.px(WIDTH), s.px(HEIGHT)));
            let at = panel_at(&spans, i, &rows, all);
            let (x, y) = (at.x + at.w / 2, at.y + at.h / 2);
            let row = match press(Some(i), x, y) {
                MenuHit::Row(r) => Some(r),
                _ => None,
            };
            assert_eq!(row, hit(&rows, at, x as i32, y as i32), "q={q}: menu {i} misses its row");
        }
    }
    set_scale(4);
}
