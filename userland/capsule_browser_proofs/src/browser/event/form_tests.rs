// NONOS Operating System (AGPL-3.0-or-later)
//! What a form sends, and what a click on a checkbox or radio button does.

use super::field_at::{field_at, Field};
use super::field_toggle::{restore, toggle};
use super::field_value::{control_value, field_text, select_values, set_control_value};
use super::form_fields::{form_fields, TooManyFields, MAX_FIELDS};
use crate::browser::dom;

fn page(html: &str) -> dom::Dom {
    dom::parse(html.as_bytes())
}

fn nth(d: &dom::Dom, tag: &str, n: usize) -> usize {
    d.nodes.iter().enumerate().filter(|(_, x)| x.tag == tag).nth(n).expect("tag present").0
}

fn send(d: &dom::Dom, submitter: Option<usize>) -> String {
    form_fields(d, nth(d, "form", 0), submitter, None).expect("a form within the limit")
}

#[test]
fn fields_go_in_document_order() {
    let d = page("<form><input name=a value=1><div><input name=b value=2></div><input name=c value=3></form>");
    assert_eq!(send(&d, None), "a=1&b=2&c=3");
}

#[test]
fn a_select_sends_the_option_it_shows() {
    let d = page(
        "<form><select name=sort><option value=new>Newest</option>\
         <option value=top selected>Top</option></select>\
         <select name=lang><option>  English \n (UK) </option><option>French</option></select>\
         <select name=tags multiple><option selected>a</option><option>b</option>\
         <option selected value=c>C</option></select></form>",
    );
    assert_eq!(send(&d, None), "sort=top&lang=English+%28UK%29&tags=a&tags=c");
    let empty = page("<form><select name=s></select></form>");
    assert_eq!(select_values(&empty, nth(&empty, "select", 0)), Vec::<String>::new());
}

#[test]
fn a_filled_in_textarea_sends_its_text_and_typing_starts_from_it() {
    let d = page("<form><textarea name=body>\nfirst line\nsecond</textarea></form>");
    let ta = nth(&d, "textarea", 0);
    assert_eq!(field_text(&d, ta), "first line\nsecond", "the newline after the tag is dropped");
    assert_eq!(send(&d, None), "body=first+line%0Asecond");
    let mut d = d;
    d.set_attr(ta, "value", "edited".into());
    assert_eq!(field_text(&d, ta), "edited", "what the reader typed wins");
}

#[test]
fn the_clicked_button_is_sent_and_disabled_controls_are_not() {
    let d = page(
        "<form><input name=q value=x><input name=off value=y disabled>\
         <button name=act value=save>Save</button><button name=act value=delete>Delete</button>\
         <input type=submit name=go value=Go><input type=reset name=r value=R></form>",
    );
    let delete = nth(&d, "button", 1);
    assert_eq!(send(&d, Some(delete)), "q=x&act=delete");
    assert_eq!(send(&d, None), "q=x", "Enter sends no button");
    let go = nth(&d, "input", 2);
    assert_eq!(send(&d, Some(go)), "q=x&go=Go");
}

/* An image submit button sends where it was clicked, name.x and name.y,
 * or x and y without a name; never its value, and not when another
 * control submitted. */
#[test]
fn an_image_button_sends_where_it_was_clicked() {
    let d =
        page("<form><input name=q value=x><input type=image name=map value=v src=m.png></form>");
    let map = nth(&d, "input", 1);
    let form = nth(&d, "form", 0);
    let sent = form_fields(&d, form, Some(map), Some((12, 7))).expect("sent");
    assert_eq!(sent, "q=x&map.x=12&map.y=7");
    assert_eq!(form_fields(&d, form, None, None).expect("sent"), "q=x", "Enter: no image");
    let d = page("<form><input type=image src=go.png></form>");
    let go = nth(&d, "input", 0);
    let sent = form_fields(&d, nth(&d, "form", 0), Some(go), Some((3, 4))).expect("sent");
    assert_eq!(sent, "x=3&y=4");
}

#[test]
fn a_form_past_the_limit_is_refused_whole() {
    let mut html = String::from("<form>");
    for i in 0..MAX_FIELDS {
        html.push_str(&format!("<input name=f{i} value={i}>"));
    }
    let d = page(&format!("{html}</form>"));
    assert!(send(&d, None).ends_with(&format!("f{}={}", MAX_FIELDS - 1, MAX_FIELDS - 1)));
    let d = page(&format!("{html}<input name=one value=more></form>"));
    assert_eq!(form_fields(&d, nth(&d, "form", 0), None, None), Err(TooManyFields));
}

#[test]
fn a_click_checks_a_checkbox_and_a_radio_takes_its_group() {
    let mut d = page(
        "<form><input type=checkbox name=keep>\
         <input type=radio name=size value=s checked><input type=radio name=size value=l></form>\
         <form><input type=radio name=size value=x checked></form>",
    );
    let (keep, small, large, other) =
        (nth(&d, "input", 0), nth(&d, "input", 1), nth(&d, "input", 2), nth(&d, "input", 3));
    assert!(matches!(field_at(&d, keep), Field::Toggle(id) if id == keep));
    assert!(toggle(&mut d, keep).is_some());
    assert_eq!(send(&d, None), "keep=on&size=s");
    let prior = toggle(&mut d, large).expect("a radio");
    assert_eq!(send(&d, None), "keep=on&size=l");
    assert!(d.nodes[small].attr("checked").is_none());
    assert!(d.nodes[other].attr("checked").is_some(), "another form's group is its own");
    restore(&mut d, &prior);
    assert_eq!(send(&d, None), "keep=on&size=s", "a cancelled click is undone");
    assert!(toggle(&mut d, keep).is_some());
    assert_eq!(send(&d, None), "size=s", "a second click unchecks");
    let mut off = page("<input type=checkbox disabled>");
    let boxed = nth(&off, "input", 0);
    assert!(toggle(&mut off, boxed).is_none());
}

#[test]
fn a_script_reads_and_sets_what_a_control_holds() {
    let mut d = page(
        "<select id=s><option value=a>A</option><optgroup><option selected>B b</option></optgroup>\
         </select><textarea>kept</textarea><input value=typed>",
    );
    let (s, ta, i) = (nth(&d, "select", 0), nth(&d, "textarea", 0), nth(&d, "input", 0));
    assert_eq!(control_value(&d, s), "B b");
    assert_eq!(control_value(&d, ta), "kept");
    assert_eq!(control_value(&d, i), "typed");
    set_control_value(&mut d, s, "a");
    assert_eq!(control_value(&d, s), "a");
    assert_eq!(select_values(&d, s), ["a"], "and a submit sends the new choice");
    set_control_value(&mut d, s, "nowhere");
    assert_eq!(control_value(&d, s), "a", "no option sends it: the choice stays");
    set_control_value(&mut d, ta, "rewritten");
    assert_eq!(field_text(&d, ta), "rewritten");
}
