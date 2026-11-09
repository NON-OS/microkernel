// NONOS Operating System (AGPL-3.0-or-later)
#![cfg(test)]
//! Tab groups and letter tiles take the toolkit's categories: a group's dot
//! is painted in its category's colour, and a tile's letter reads on its fill.

use nonos_toolkit::paint::PaintBuffer;
use nonos_toolkit::theme::{Category, CATEGORIES};
use nonos_toolkit::widgets::Controls;

use crate::browser::shell::kept::Group;
use crate::browser::shell::Shell;
use crate::browser::sidebar::paint::paint;
use crate::browser::tokens::category::category_on;
use crate::browser::tokens::{category, Palette, DARK, LIGHT};
use crate::browser::ui::group_rows::dot_slot;
use crate::browser::ui::layout::{compute, SidebarMode};
use crate::browser::ui::sidebar_geom::{rows, RowKind};

use super::contrast_tests::{contrast, every_system_palette};

const W: i32 = 1400;
const H: i32 = 1400;

/* Stored colours past the eight keep the slot `color % 8` always gave them.
 * The appearance tests switch the process-wide mode while this paints, so
 * the dot may be the category under any appearance, but never another one. */
#[test]
fn a_group_dot_is_painted_in_its_category_colour() {
    let colours = [0u8, 1, 2, 3, 4, 5, 6, 13];
    let mut shell = Shell::default();
    for (id, &color) in colours.iter().enumerate() {
        shell.groups.push(Group { id: id as u8, name: format!("G{id}"), color });
    }
    let lay = compute(W, H, SidebarMode::Full);
    let mut px = vec![0u32; (W * H) as usize];
    let mut fb =
        PaintBuffer { pixels: &mut px, stride_words: W as u32, width: W as u32, height: H as u32 };
    let mut c = Controls::acting_on_press();
    paint(&shell, None, &lay, &mut fb, &mut c);
    let mut seen = 0;
    for r in rows(false, lay.sidebar, colours.len()) {
        let RowKind::Group(i) = r.kind else { continue };
        let d = dot_slot(r.rect).0;
        let at = px[((d.y + d.h / 2) * W + d.x + d.w / 2) as usize];
        let k = Category::new(colours[i] as usize);
        let want = [category(k).0, category_on(&LIGHT, k).0, category_on(&DARK, k).0];
        assert!(want.contains(&at), "group {i}, colour {}: {at:08X}", colours[i]);
        seen += 1;
    }
    assert_eq!(seen, colours.len());
}

fn every_palette() -> Vec<(String, Palette)> {
    let mut all = vec![("light".to_string(), LIGHT), ("dark".to_string(), DARK)];
    all.extend(every_system_palette());
    all
}

/* A tile's letter and a chip's name are text: 4.5:1 on the fill. The fill
 * itself is a graphic beside the chrome: 3:1 on every ground it sits on. */
#[test]
fn letter_tiles_read_on_their_fill_under_every_palette() {
    for (name, p) in every_palette() {
        for i in 0..CATEGORIES {
            let (fill, ink) = category_on(&p, Category::new(i));
            let r = contrast(ink, fill);
            assert!(r >= 4.5, "{name}: ink {ink:08X} on tile {i} {fill:08X} is {r:.2}:1");
            for g in [p.ground, p.surface, p.sidebar, p.field] {
                let r = contrast(fill, g);
                assert!(r >= 3.0, "{name}: tile {i} {fill:08X} on {g:08X} is {r:.2}:1");
            }
        }
    }
}
