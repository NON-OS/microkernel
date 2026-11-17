// NONOS Operating System (AGPL-3.0-or-later)
//! A select's list: what it holds, how the keyboard moves through it,
//! where it is drawn, and what a choice does to what the form sends.

use super::field_value::{choose_option, select_values};
use super::select_list::{SelectList, MAX_ROWS};
use crate::browser::dom;

fn select_of(d: &dom::Dom) -> usize {
    d.nodes.iter().position(|n| n.tag == "select").expect("a select")
}

const SIZES: &[u8] = b"<select name=s><option>Small<option selected value=m>Medium\
    <option disabled>Gone<optgroup label=Big><option>Large<option>Huge</optgroup></select>";

#[test]
fn the_list_holds_the_options_and_highlights_the_shown_one() {
    let d = dom::parse(SIZES);
    let list = SelectList::open(&d, select_of(&d)).expect("a list");
    let labels: Vec<&str> = list.rows.iter().map(|r| r.label.as_str()).collect();
    assert_eq!(labels, ["Small", "Medium", "Gone", "Big", "Large", "Huge"]);
    assert!(list.rows[3].heading && !list.rows[3].choosable(), "a heading is not chosen");
    assert!(!list.rows[2].choosable(), "a disabled option is not chosen");
    assert_eq!(list.rows[list.hi].label, "Medium");
    assert!(!list.multiple);
}

#[test]
fn the_keys_skip_what_cannot_be_chosen_and_stop_at_the_ends() {
    let d = dom::parse(SIZES);
    let mut list = SelectList::open(&d, select_of(&d)).expect("a list");
    list.step(1);
    assert_eq!(list.rows[list.hi].label, "Large", "past Gone and the heading");
    list.step(5);
    assert_eq!(list.rows[list.hi].label, "Huge", "stops at the end");
    list.step(-2);
    assert_eq!(list.rows[list.hi].label, "Medium");
    list.to_end(false);
    assert_eq!(list.highlighted().map(|id| d.nodes[id].tag.as_str()), Some("option"));
    assert_eq!(list.rows[list.hi].label, "Small");
    list.to_end(true);
    assert_eq!(list.rows[list.hi].label, "Huge");
}

#[test]
fn nothing_opens_for_a_disabled_or_empty_select() {
    let d = dom::parse(b"<select disabled><option>a</select>");
    assert!(SelectList::open(&d, select_of(&d)).is_none());
    let d = dom::parse(b"<select></select>");
    assert!(SelectList::open(&d, select_of(&d)).is_none());
}

#[test]
fn a_long_list_scrolls_to_keep_the_highlight_in_view() {
    let html: String = (0..40).map(|i| format!("<option>o{i}")).collect();
    let d = dom::parse(format!("<select>{html}</select>").as_bytes());
    let mut list = SelectList::open(&d, select_of(&d)).expect("a list");
    assert_eq!(list.shown(), MAX_ROWS);
    list.step(20);
    assert_eq!(list.hi, 20);
    assert_eq!(list.first, 20 + 1 - MAX_ROWS, "the highlight is the last row shown");
    list.to_end(false);
    assert_eq!(list.first, 0);
}

#[test]
fn a_choice_is_what_the_form_sends_and_says_whether_it_changed() {
    let mut d = dom::parse(SIZES);
    let s = select_of(&d);
    let list = SelectList::open(&d, s).expect("a list");
    let large = list.rows[4].id;
    let medium = list.rows[1].id;
    assert!(choose_option(&mut d, s, large));
    assert_eq!(select_values(&d, s), ["Large"]);
    assert!(!choose_option(&mut d, s, large), "the same choice again changes nothing");
    assert!(choose_option(&mut d, s, medium));
    assert_eq!(select_values(&d, s), ["m"]);
    let p = d.nodes.iter().position(|n| n.tag == "select").unwrap_or(0);
    assert!(!choose_option(&mut d, p, p), "only an option of this select");
}

#[test]
fn a_multiple_select_turns_options_on_and_off() {
    let mut d = dom::parse(b"<select multiple><option>a<option>b<option>c</select>");
    let s = select_of(&d);
    let mut list = SelectList::open(&d, s).expect("a list");
    assert!(list.multiple);
    let (a, c) = (list.rows[0].id, list.rows[2].id);
    assert!(choose_option(&mut d, s, a) && choose_option(&mut d, s, c));
    assert_eq!(select_values(&d, s), ["a", "c"]);
    assert!(choose_option(&mut d, s, a));
    assert_eq!(select_values(&d, s), ["c"]);
    list.refresh(&d);
    let on: Vec<bool> = list.rows.iter().map(|r| r.selected).collect();
    assert_eq!(on, [false, false, true]);
}
