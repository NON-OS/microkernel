// NONOS Operating System (AGPL-3.0-or-later)
//! Split view: two halves and a divider covering the window, the lower tab
//! on the left whichever has the keys, and both panes kept on screen as tabs
//! move and close.

use super::split::{after_close, halves, retarget, right_of, sides, DIVIDER};

#[test]
fn the_halves_and_the_divider_cover_the_window() {
    for w in [1240, 1241, 640, 3] {
        let ((lx, lw), (rx, rw)) = halves(w);
        assert_eq!(lx, 0);
        assert_eq!(rx, lw + DIVIDER, "{w}");
        assert_eq!(rx + rw, w.max(DIVIDER), "{w}");
        assert!(lw.abs_diff(rw) <= 1, "{w}: {lw} and {rw}");
    }
}

#[test]
fn the_lower_tab_is_on_the_left_whichever_has_the_keys() {
    assert_eq!(sides(0, 3), (0, 3));
    assert_eq!(sides(3, 0), (0, 3));
}

#[test]
fn moving_onto_the_split_trades_places_with_it() {
    assert_eq!(retarget(Some(2), 0, 2), Some(0), "both stay on screen");
    assert_eq!(retarget(Some(2), 0, 1), Some(2), "another tab in front, the split stays");
    assert_eq!(retarget(None, 0, 1), None);
}

#[test]
fn closing_a_pane_leaves_the_other_in_front() {
    assert_eq!(after_close(Some(3), 1, 3), 2, "the split was after it: one lower");
    assert_eq!(after_close(Some(0), 2, 3), 0);
    assert_eq!(after_close(None, 4, 4), 3, "no split: the new last tab");
    assert_eq!(after_close(None, 1, 4), 1);
}

#[test]
fn a_click_is_over_the_pane_it_lands_in() {
    let ((_, lw), (rx, _)) = halves(1240);
    assert!(!right_of(10, 1240) && !right_of(lw as i32 - 1, 1240));
    assert!(right_of(rx as i32, 1240) && right_of(1239, 1240));
    assert!(!right_of(-5, 1240));
}
