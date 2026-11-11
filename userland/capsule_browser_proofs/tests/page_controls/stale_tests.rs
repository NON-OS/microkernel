// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use capsule_browser_proofs::browser::shell::Shell;
use capsule_browser_proofs::browser::ui::page_key::live;
use capsule_browser_proofs::browser::ui::page_part::PagePart;
use nonos_toolkit::widgets::{Controls, Step};

use super::fixtures::{bookmarks, downloads, history, tabs};
use super::found::{mark, new_tab};
use super::kit::{at_scale, mid, paint, spots};

/* What a press on the control `pick` finds in `s` acts for, as drawn. */
fn pressed(s: &Shell, pick: fn(&PagePart) -> bool) -> PagePart {
    let mut c = Controls::acting_on_press();
    paint(s, &mut c);
    let (part, r) = spots(&c).into_iter().find(|(p, _)| pick(p)).expect("no such control");
    let (x, y) = mid(r);
    assert_eq!(c.pressed(x, y), Step::Act(part));
    part
}

/* The press stays good on `s` as drawn, and goes dead once `change` moves
 * another entry into its index or takes its entry away. */
fn stale(s: fn() -> Shell, pick: fn(&PagePart) -> bool, change: &[fn(&mut Shell)]) {
    let part = pressed(&s(), pick);
    assert!(live(&s(), part), "{part:?} dead on the list it was drawn from");
    for f in change {
        let mut moved = s();
        f(&mut moved);
        assert!(!live(&moved, part), "{part:?} acts on what moved into its place");
    }
}

#[test]
fn a_press_on_a_moved_or_gone_entry_does_not_act_on_what_took_its_place() {
    for q in [4, 5, 6, 8] {
        let _g = at_scale(q);
        let rm1: fn(&PagePart) -> bool = |p| matches!(p, PagePart::Remove(1, _));
        let first = |s: &mut Shell| s.bookmarks.insert(0, mark(0));
        let gone = |s: &mut Shell| {
            s.bookmarks.remove(1);
        };
        stale(bookmarks, rm1, &[first, gone, |s| s.bookmarks.truncate(1)]);
        let open: fn(&PagePart) -> bool = |p| matches!(p, PagePart::Open(2, _));
        let early = |s: &mut Shell| {
            s.visits.remove(0);
        };
        stale(history, open, &[early, |s| s.visits.truncate(2)]);
        let dl: fn(&PagePart) -> bool = |p| matches!(p, PagePart::Remove(0, _));
        stale(downloads, dl, &[|s| s.downloads.insert(0, downloads().downloads.remove(2))]);
        let close: fn(&PagePart) -> bool = |p| matches!(p, PagePart::Close(2, _));
        let opened = |s: &mut Shell| s.cards.insert(0, new_tab());
        let shut = |s: &mut Shell| {
            s.cards.remove(2);
        };
        stale(|| tabs(false), close, &[opened, shut]);
    }
}
