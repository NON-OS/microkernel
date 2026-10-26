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

//! What each swap and take leaves, crashed or held or refused.

use super::*;

#[test]
fn a_crash_at_any_step_serves_the_old_pack_nothing_or_the_new() {
    let (old, new) = (Some(b"old".to_vec()), Some(b"new".to_vec()));
    let mut seen = Vec::new();
    for steps in 0..8 {
        let mut m = Model { steps: Some(steps), ..with_old() };
        let _ = swap(&mut m, "vim", b"new", &minted(b"new"));
        let now = served(&m);
        assert!(now == old || now.is_none() || now == new, "after {steps} steps: {now:?}");
        seen.push(now);
        m.steps = None;
        assert_eq!(swap(&mut m, "vim", b"new", &minted(b"new")), Ok(()), "retried after {steps}");
        assert_eq!(served(&m), new);
        assert!(!m.files.contains_key(&new_path("vim")));
    }
    assert!(seen.contains(&old) && seen.contains(&None) && seen.contains(&new));
}

#[test]
fn a_held_pack_is_neither_replaced_nor_removed() {
    let held = Model { held: Some(pack_path("vim")), ..with_old() };
    let mut m = held.clone();
    assert_eq!(swap(&mut m, "vim", b"new", &minted(b"new")), Err(Held::InUse));
    assert_eq!(m, held, "no .new is left, the old pack serves on");
    assert_eq!(take(&mut m, "vim"), Err(Held::InUse));
    assert_eq!(m, held);
}

#[test]
fn a_pack_the_store_will_not_take_leaves_the_old_one_serving() {
    let full = Model { full: true, ..with_old() };
    let mut m = full.clone();
    assert_eq!(swap(&mut m, "vim", b"new", &minted(b"new")), Err(Held::Unwritten));
    assert_eq!(m, full);
    assert_eq!(served(&m), Some(b"old".to_vec()));
}

#[test]
fn taking_a_pack_out_leaves_nothing_of_it() {
    let mut m = Model { steps: Some(2), ..with_old() };
    let _ = swap(&mut m, "vim", b"new", &minted(b"new"));
    m.steps = None;
    assert_eq!(take(&mut m, "vim"), Ok(()));
    assert!(m.files.is_empty(), "{:?}", m.files.keys());
    assert_eq!(take(&mut m, "vim"), Ok(()), "a package with no pack is out");
}
