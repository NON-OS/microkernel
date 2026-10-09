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

//! The header's Forward arrow: it goes back down into the folders Back went
//! up out of, and nowhere once the person has gone somewhere else.

use alloc::string::String;
use alloc::vec::Vec;

use crate::fm_logic::nav_trail::Trail;
use crate::fm_logic::tags::TagMap;
use crate::fm_logic::tags_toggle::toggle_tag;

#[test]
fn forward_retraces_each_step_back() {
    let mut t = Trail::default();
    assert!(!t.can_forward(), "a fresh window has nowhere to go forward to");
    t.went_up("/a/b/c/");
    t.went_up("/a/b/");
    assert!(t.can_forward());
    assert_eq!(t.forward().as_deref(), Some("/a/b/"));
    assert_eq!(t.forward().as_deref(), Some("/a/b/c/"));
    assert_eq!(t.forward(), None);
    assert!(!t.can_forward());
}

#[test]
fn opening_the_folder_back_left_keeps_the_trail() {
    let mut t = Trail::default();
    t.went_up("/a/b/c/");
    t.went_up("/a/b/");
    t.arrived("/a/b/");
    assert_eq!(t.forward().as_deref(), Some("/a/b/c/"));
}

#[test]
fn going_anywhere_else_ends_the_trail() {
    let mut t = Trail::default();
    t.went_up("/a/b/");
    t.arrived("/downloads/");
    assert!(!t.can_forward());
}

fn paths(list: &[&str]) -> Vec<String> {
    list.iter().map(|p| String::from(*p)).collect()
}

#[test]
fn a_tag_goes_on_every_selected_entry() {
    let mut m = TagMap::default();
    let sel = paths(&["/a.txt", "/b.txt", "/c/"]);
    assert_eq!(toggle_tag(&mut m, &sel, "Work"), b"tagged");
    for p in &sel {
        assert_eq!(m.tags_for(p), alloc::vec!["work"]);
    }
}

#[test]
fn a_tag_all_of_them_carry_comes_off_all() {
    let mut m = TagMap::default();
    let sel = paths(&["/a.txt", "/b.txt"]);
    toggle_tag(&mut m, &sel, "work");
    assert_eq!(toggle_tag(&mut m, &sel, "WORK"), b"untagged");
    assert!(m.tags_for("/a.txt").is_empty() && m.tags_for("/b.txt").is_empty());
}

#[test]
fn some_untagged_means_tag_the_rest_not_untag() {
    let mut m = TagMap::default();
    toggle_tag(&mut m, &paths(&["/a.txt"]), "work");
    let sel = paths(&["/a.txt", "/b.txt"]);
    assert_eq!(toggle_tag(&mut m, &sel, "work"), b"tagged");
    assert_eq!(m.tags_for("/b.txt"), alloc::vec!["work"]);
    assert_eq!(m.tags_for("/a.txt"), alloc::vec!["work"]);
}

#[test]
fn a_bad_name_is_refused_and_said() {
    let mut m = TagMap::default();
    let said = toggle_tag(&mut m, &paths(&["/a.txt"]), "has space");
    assert!(said.starts_with(b"not tagged"));
    assert!(m.tags_for("/a.txt").is_empty());
}
