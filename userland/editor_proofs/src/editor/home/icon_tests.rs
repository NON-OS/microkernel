// NONOS Operating System (AGPL-3.0-or-later)
//! Home's document rows lead with the toolkit's file icon: in each row, the
//! `ICON` square `SPACE.lg` in from its left edge is inked over the row's
//! ground, at 1x and 2x, and below the separator the ink stays in that
//! square's rows.

use nonos_toolkit::design::scale::{scale, set_scale};
use nonos_toolkit::design::size::{ICON, SPACE};
use nonos_toolkit::paint::PaintBuffer;
use nonos_toolkit::widgets::{Card, Controls, Rect};

use super::doc_ref::{section_title, DocRef};
use super::metrics_pane::{doc_row, list_card};
use super::screen::paint_screen;
use super::state::HomeState;
use crate::editor::square_tests::lock;
use crate::editor::strip_tests::W;

const H: u32 = 720;
const DOCS: [(&str, &str); 2] = [("notes.txt", "/notes.txt"), ("main.rs", "/src/main.rs")];

#[test]
fn every_document_row_leads_with_the_file_icon() {
    let _g = lock();
    for q in [4, 8] {
        (set_scale(q), HomeState::select(1));
        let list: Vec<DocRef> = DOCS.iter().map(|&(n, p)| DocRef { name: n.into(), path: p.into() }).collect();
        let mut px = vec![0u32; (W * H) as usize];
        let mut fb = PaintBuffer { pixels: &mut px, stride_words: W, width: W, height: H };
        paint_screen(&mut fb, 1, &list, &mut Controls::acting_on_press());
        let title = section_title(1);
        let content = Card::new().title(title).content(list_card(W, H, title, DOCS.len()).0);
        let (pad, side) = (scale().px(u32::from(SPACE.lg)), scale().px(ICON));
        for i in 0..DOCS.len() {
            let row = doc_row(content, i);
            let hair = scale().hair();
            let sq = Rect { x: row.x + pad, y: row.y + hair, w: side, h: row.h - hair };
            let dots = (sq.y..sq.y + sq.h).flat_map(|y| (sq.x..sq.x + sq.w).map(move |x| (x, y)));
            let mid = (row.y + row.h / 2) as i64;
            let ground = px[(mid as u32 * W + row.x + pad / 2) as usize];
            let inked: Vec<u32> = dots.filter(|&(x, y)| px[(y * W + x) as usize] != ground).map(|d| d.1).collect();
            let centred = inked.iter().all(|&y| (y as i64 - mid).abs() <= side as i64 / 2 + 1);
            assert!(!inked.is_empty() && centred, "row {i} at {q}: {} inked", inked.len());
        }
    }
    (HomeState::select(0), set_scale(4));
}
