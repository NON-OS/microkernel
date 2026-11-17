// NONOS Operating System (AGPL-3.0-or-later)
//! The keyboard over the command palette: stepping the way the menu's arrow
//! keys do keeps the selection among the rows the menu shows, at 1x and 2x,
//! and the row washed is the one Enter acts on.

use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::widgets::{Menu, Pointer};

use crate::palette_boxes::{cases, entries, mid};
use crate::palette_view::{chosen, items, lit, matches, view};
use crate::strip_boxes::serial;

#[test]
fn the_washed_row_is_the_one_enter_acts_on() {
    let _g = serial();
    let all = entries();
    for (q, body, needle) in cases() {
        set_scale(q);
        let m = matches(&all, needle.as_bytes());
        let rows = items(&m);
        let v = view(body, &rows);
        assert!(v.rows <= m.len() && (v.rows > 0 || m.is_empty()), "{needle:?} at {q}");
        let mut sel = 0;
        for step in [1, 1, -1, 3, -5, 9, 2] {
            sel = Menu::next(&rows[..v.rows], chosen(v.rows, sel), step).unwrap_or(0);
            let washed = lit(&v, &Pointer::new(), sel).0;
            assert_eq!(washed, chosen(v.rows, sel), "{needle:?} after {step} at {q}");
            assert!(washed.map_or(m.is_empty(), |i| i < v.rows && i == sel));
            let (Some(i), Some(x)) = (washed, washed.map(|i| mid(&v, i).0)) else { continue };
            assert_eq!(
                v.menu.hit(v.at, x, mid(&v, i).1),
                Some(i),
                "{needle:?} {i} is drawn at {q}"
            );
        }
        let past = lit(&v, &Pointer::new(), m.len() + 4).0;
        assert_eq!(
            past,
            v.rows.checked_sub(1),
            "{needle:?}: a stale selection washes the last row shown"
        );
    }
    set_scale(4);
}
