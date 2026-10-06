// NONOS Operating System (AGPL-3.0-or-later)
//! A click on a label reaches the control it names: by `for`, else the
//! first control inside it, and nothing a label cannot stand for.

use super::label_for::label_control;
use crate::browser::dom;

fn by_id(d: &dom::Dom, id: &str) -> usize {
    d.nodes.iter().position(|n| n.attr("id") == Some(id)).expect("id present")
}

#[test]
fn for_names_the_control() {
    let d = dom::parse(
        b"<label id=l for=agree>I <b id=b>agree</b></label><input id=agree type=checkbox>",
    );
    let agree = by_id(&d, "agree");
    assert_eq!(label_control(&d, by_id(&d, "l")), Some(agree));
    assert_eq!(label_control(&d, by_id(&d, "b")), Some(agree), "from text inside the label");
}

#[test]
fn a_nested_control_is_found_without_for() {
    let d =
        dom::parse(b"<label id=l><span>Name</span><input type=hidden id=h><input id=name></label>");
    assert_eq!(label_control(&d, by_id(&d, "l")), Some(by_id(&d, "name")), "hidden is skipped");
    let d = dom::parse(b"<label id=l>Size <select id=s><option>S</select></label>");
    assert_eq!(label_control(&d, by_id(&d, "l")), Some(by_id(&d, "s")));
}

#[test]
fn nothing_a_label_cannot_stand_for() {
    let d = dom::parse(b"<label id=l for=x>X</label><div id=x></div>");
    assert_eq!(label_control(&d, by_id(&d, "l")), None, "a div is not labelable");
    let d = dom::parse(b"<label id=l for=gone>X</label>");
    assert_eq!(label_control(&d, by_id(&d, "l")), None, "no such id");
    let d = dom::parse(b"<p id=p>no label</p><input>");
    assert_eq!(label_control(&d, by_id(&d, "p")), None);
}
