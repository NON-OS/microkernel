// NONOS Operating System (AGPL-3.0-or-later)
//! What a typed character meets in a form field: the page's maxlength,
//! refused as the page asked, and the browser's own bound, which is far past
//! the 512 bytes every field used to stop at without a word.

use super::field_room::{room, Room, INPUT_MAX, TEXTAREA_MAX};
use crate::browser::dom;

fn field(html: &str, tag: &str) -> (dom::Dom, usize) {
    let d = dom::parse(html.as_bytes());
    let id = d.nodes.iter().position(|n| n.tag == tag).expect("tag present");
    (d, id)
}

#[test]
fn a_comment_runs_past_the_old_limit() {
    let (d, id) = field("<form><textarea></textarea></form>", "textarea");
    let written = "a".repeat(5_000);
    assert_eq!(room(&d.nodes[id], &written, 'b'), Room::Fits);
    let (d, id) = field("<form><input name=q></form>", "input");
    assert_eq!(room(&d.nodes[id], &"a".repeat(600), 'b'), Room::Fits);
}

#[test]
fn the_pages_maxlength_counts_characters() {
    let (d, id) = field("<form><input maxlength=3></form>", "input");
    let n = &d.nodes[id];
    assert_eq!(room(n, "ab", 'c'), Room::Fits);
    assert_eq!(room(n, "abc", 'd'), Room::PageLimit);
    assert_eq!(room(n, "éé", 'é'), Room::Fits, "three characters, six bytes");
    let (d, id) = field("<form><input maxlength=lots></form>", "input");
    assert_eq!(room(&d.nodes[id], "abcdef", 'g'), Room::Fits, "not a number: no limit");
}

#[test]
fn the_browsers_bound_is_reported_with_its_size() {
    let (d, id) = field("<form><input></form>", "input");
    let full = "a".repeat(INPUT_MAX);
    assert_eq!(room(&d.nodes[id], &full, 'b'), Room::BrowserLimit(INPUT_MAX));
    let (d, id) = field("<form><textarea></textarea></form>", "textarea");
    let full = "a".repeat(TEXTAREA_MAX - 1);
    assert_eq!(room(&d.nodes[id], &full, 'é'), Room::BrowserLimit(TEXTAREA_MAX), "by bytes");
    assert_eq!(room(&d.nodes[id], &full, 'b'), Room::Fits);
}
