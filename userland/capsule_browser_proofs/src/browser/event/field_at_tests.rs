// NONOS Operating System (AGPL-3.0-or-later)
//! What a click on a form control does: only a submit button submits.

use super::field_at::{field_at, Field};
use crate::browser::dom;

fn first(html: &str, tag: &str) -> (dom::Dom, usize) {
    let d = dom::parse(html.as_bytes());
    let id = d.nodes.iter().position(|n| n.tag == tag).expect("tag present");
    (d, id)
}

#[test]
fn a_type_button_does_not_submit() {
    let (d, id) = first("<form><button type=button>x</button></form>", "button");
    assert!(matches!(field_at(&d, id), Field::None));
    let (d, id) = first("<form><button type=reset>x</button></form>", "button");
    assert!(matches!(field_at(&d, id), Field::None));
}

#[test]
fn a_plain_or_submit_button_submits() {
    let (d, id) = first("<form><button>go</button></form>", "button");
    assert!(matches!(field_at(&d, id), Field::Submit(n) if n == id));
    let (d, id) = first("<form><button type=SUBMIT>go</button></form>", "button");
    assert!(matches!(field_at(&d, id), Field::Submit(n) if n == id));
}

#[test]
fn a_text_input_takes_focus() {
    let (d, id) = first("<form><input name=q></form>", "input");
    assert!(matches!(field_at(&d, id), Field::Edit(n) if n == id));
}

/* A click on a select, or on the text of the option it shows, opens it. */
#[test]
fn a_select_opens() {
    let (d, id) = first("<form><select><option>a</select></form>", "select");
    assert!(matches!(field_at(&d, id), Field::Select(n) if n == id));
    let text = d.nodes.iter().position(|n| n.text == "a").expect("the option's text");
    assert!(matches!(field_at(&d, text), Field::Select(n) if n == id));
}
