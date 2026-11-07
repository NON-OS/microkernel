// NONOS Operating System (AGPL-3.0-or-later)
//! The pointer over the command palette: a click picks the entry its row
//! shows under every query, a press dragged off its row picks nothing, a
//! press off the palette closes it, and leaving lets go of a held row.

use nonos_app_skeleton::InputKind::{ButtonDown, ButtonUp, PointerAbs};

use crate::palette::filter;
use crate::palette_boxes::{entries, ev, mid, paint, typed, BODIES, NEEDLES};
use crate::palette_ptr::{feed, leave, Step};
use crate::palette_view::{chosen, items, lit, matches, view};
use crate::strip_boxes::serial;

#[test]
fn a_click_picks_the_entry_its_row_shows() {
    let _g = serial();
    let all = entries();
    for needle in NEEDLES {
        let mut hits = [0usize; crate::palette::MAX_ROWS];
        let n = filter(&all, needle.as_bytes(), &mut hits);
        let m = matches(&all, needle.as_bytes());
        let rows = items(&m);
        let v = view(BODIES[1], &rows);
        assert_eq!(m.len(), n, "{needle:?}");
        for i in 0..v.rows {
            assert_eq!((m[i].label, m[i].action), (all[hits[i]].label, all[hits[i]].action));
            let mut pal = typed(needle);
            assert_eq!(feed(&mut pal, &v, &ev(ButtonDown, mid(&v, i))), Step::Repaint);
            assert_eq!(feed(&mut pal, &v, &ev(ButtonUp, mid(&v, i))), Step::Pick, "{needle:?} {i}");
            let picked = m[chosen(v.rows, pal.sel).unwrap()];
            assert_eq!(picked.action, all[hits[i]].action, "{needle:?} row {i}");
            assert_eq!(picked.label, all[hits[i]].label, "{needle:?} row {i}");
        }
    }
}

#[test]
fn a_press_dragged_off_its_row_picks_nothing_and_one_outside_closes() {
    let _g = serial();
    let all = entries();
    let m = matches(&all, b"");
    let rows = items(&m);
    let v = view(BODIES[0], &rows);
    let mut pal = typed("");
    feed(&mut pal, &v, &ev(ButtonDown, mid(&v, 1)));
    assert_eq!(feed(&mut pal, &v, &ev(PointerAbs, mid(&v, 2))), Step::Repaint);
    assert_ne!(feed(&mut pal, &v, &ev(ButtonUp, mid(&v, 2))), Step::Pick);
    let field = ((v.field.x + 4) as i32, (v.field.y + v.field.h / 2) as i32);
    assert_ne!(feed(&mut pal, &v, &ev(ButtonDown, field)), Step::Close);
    let below = (v.at.x as i32 - 2, (v.at.y + v.at.h / 2) as i32);
    assert_eq!(feed(&mut pal, &v, &ev(ButtonDown, below)), Step::Close);
}

#[test]
fn leaving_lets_go_of_a_held_row_once_and_lights_nothing_more() {
    let _g = serial();
    let all = entries();
    let m = matches(&all, b"e");
    let rows = items(&m);
    let v = view(BODIES[0], &rows);
    let mut pal = typed("e");
    feed(&mut pal, &v, &ev(ButtonDown, mid(&v, 2)));
    assert_eq!(lit(&v, &pal.ptr, pal.sel), (Some(2), Some(2)));
    assert!(leave(&mut pal, &v));
    assert_eq!(lit(&v, &pal.ptr, pal.sel), (Some(2), None));
    let mut fresh = typed("e");
    fresh.sel = 2;
    assert!(paint(BODIES[0], &pal, &all) == paint(BODIES[0], &fresh, &all));
    assert!(!leave(&mut pal, &v));
}
