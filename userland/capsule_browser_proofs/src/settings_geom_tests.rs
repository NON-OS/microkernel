// NONOS Operating System (AGPL-3.0-or-later)
#![cfg(test)]
//! The settings page's geometry: every control hits where it is drawn at
//! any width, and nothing a row holds overlaps another or leaves the frame.

use nonos_toolkit::paint::PaintBuffer;
use nonos_toolkit::widgets::Controls;

use crate::browser::settings::controls::paint;
use crate::browser::settings::look::Shows;
use crate::browser::ui::rect::Rect;
use crate::browser::ui::settings_geom::{Control, Item, Kind, SettingsGeom};

const WIDTHS: [i32; 5] = [360, 400, 800, 1100, 1600];

pub fn centre(r: Rect) -> (i32, i32) {
    (r.x + r.w / 2, r.y + r.h / 2)
}

fn overlap(a: Rect, b: Rect) -> bool {
    a.x < b.right() && b.x < a.right() && a.y < b.bottom() && b.y < a.bottom()
}

/// The controls the page lays out over `g`, `h` tall, with every button
/// live, as a paint leaves them for a press.
pub fn hits(g: &SettingsGeom, h: i32) -> Controls<Control> {
    let had = Shows { size: 1, proxy: true, history: true, ..Shows::default() };
    let live = Shows { downloads: true, bookmarks: 1, ..had };
    let (w, h) = (g.frame.right() as u32, h as u32);
    let mut px = vec![0u32; (w * h) as usize];
    let mut fb = PaintBuffer { pixels: &mut px, stride_words: w, width: w, height: h };
    let mut c = Controls::acting_on_press();
    paint(&mut fb, g, h as i32, &live, &mut c);
    c
}

pub fn controls(g: &SettingsGeom) -> Vec<(Control, Rect)> {
    let ctl = |it: &Item| match it.kind {
        Kind::Ctl(c) => Some((c, it.rect)),
        _ => None,
    };
    g.items.iter().filter_map(ctl).collect()
}

#[test]
fn every_control_hits_at_its_centre_at_every_width() {
    for w in WIDTHS {
        let g = SettingsGeom::new(w, 4000, 0);
        let (all, c) = (controls(&g), hits(&g, 4000));
        assert!(all.len() >= 20, "{w}: {}", all.len());
        for (ctl, r) in all {
            assert!(r.w > 0 && r.h > 0, "{w} {ctl:?} {r:?}");
            let (x, y) = centre(r);
            assert_eq!(c.at(x, y), Some(ctl), "{w} {ctl:?}");
            assert_eq!(g.rect(ctl), Some(r), "{w} {ctl:?}");
        }
    }
}

#[test]
fn nothing_but_cards_overlaps_and_all_sits_inside_the_frame() {
    for w in WIDTHS {
        let g = SettingsGeom::new(w, 4000, 0);
        let parts: Vec<_> = g.items.iter().filter(|it| it.kind != Kind::Card).collect();
        for (i, a) in parts.iter().enumerate() {
            assert!(a.rect.x >= g.frame.x && a.rect.right() <= g.frame.right(), "{w} {a:?}");
            for b in &parts[i + 1..] {
                let empty = a.rect.w == 0 || b.rect.w == 0;
                assert!(empty || !overlap(a.rect, b.rect), "{w}: {a:?} over {b:?}");
            }
        }
    }
}
