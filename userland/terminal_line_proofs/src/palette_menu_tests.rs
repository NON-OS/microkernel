// NONOS Operating System (AGPL-3.0-or-later)
//! The command palette drawn as the shared menu: at 1x and 2x, over a wide
//! and a narrow body, every pixel a hovered or held row lights lies where a
//! click on that row lands.

use nonos_app_skeleton::InputKind::{ButtonDown, PointerAbs};
use nonos_toolkit::design::scale::set_scale;

use crate::palette_boxes::{cases, changed, entries, ev, mid, paint, typed};
use crate::palette_ptr::feed;
use crate::palette_view::{items, matches, view};
use crate::strip_boxes::serial;

#[test]
fn every_lit_pixel_is_where_the_click_lands() {
    let _g = serial();
    let all = entries();
    for (q, body, needle) in cases() {
        set_scale(q);
        let m = matches(&all, needle.as_bytes());
        let rows = items(&m);
        let v = view(body, &rows);
        for i in 0..v.rows {
            let mut pal = typed(needle);
            pal.sel = (i + 1) % v.rows;
            let other = paint(body, &pal, &all);
            feed(&mut pal, &v, &ev(PointerAbs, mid(&v, i)));
            assert_eq!(pal.sel, i, "hovering row {i} of {needle:?} selects it");
            let hovered = paint(body, &pal, &all);
            let lit = changed(&v, &other, &hovered);
            let j = (i + 1) % v.rows;
            assert!(
                v.rows == 1 || lit.contains(&Some(i)),
                "row {i} of {needle:?} lights nothing at {q}"
            );
            assert!(
                lit.iter().all(|&r| r == Some(i) || r == Some(j)),
                "{needle:?} {i} at {q}: {lit:?}"
            );
            feed(&mut pal, &v, &ev(ButtonDown, mid(&v, i)));
            let held = changed(&v, &hovered, &paint(body, &pal, &all));
            assert!(
                !held.is_empty() && held.iter().all(|&r| r == Some(i)),
                "{needle:?} {i} at {q}"
            );
        }
    }
    set_scale(4);
}
