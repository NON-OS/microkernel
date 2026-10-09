// NONOS Operating System (AGPL-3.0-or-later)
//! Form pseudo-classes evaluated from markup, where :enabled, :optional and
//! :link used to hold for every element and the rest never held. Expected
//! counts are Chromium's querySelectorAll on this form.

use capsule_browser_proofs::browser::{css, dom};

const FORM: &str = "<!DOCTYPE html><html lang=en><head></head><body><form id=f>\
<input type=text placeholder=Search class=a><input type=text value=v placeholder=P class=b>\
<input type=checkbox checked class=c><input type=checkbox class=d>\
<input type=radio name=r checked class=e><input type=radio name=r class=e2>\
<input type=radio name=q class=f><input type=email required class=g>\
<input type=email value='not an address' class=g2><input type=url value=x.org class=g3>\
<input type=text disabled class=h><input type=text readonly class=i>\
<input type=number min=1 max=3 value=5 class=j><input type=number min=0 step=2 value=3 class=j2>\
<input type=range min=0 max=10 value=50 class=j3><input type=SUBMIT class=k value=Go>\
<button class=l>B</button><button class=m disabled>D</button><button type=button class=m2>X</button>\
<select class=n><option>1</option><option selected>2</option></select>\
<select class=n2 required><option value=''>pick</option><option>a</option></select>\
<textarea class=o></textarea><fieldset disabled><legend><input class=p1></legend>\
<input class=p></fieldset><progress class=pr></progress></form>\
<div contenteditable class=ce><span class=cs>x</span></div></body></html>";

fn check(cases: &[(&str, usize)]) {
    let d = dom::parse(FORM.as_bytes());
    for (sel, want) in cases {
        assert_eq!(css::select(&d, sel, usize::MAX).len(), *want, "{sel}");
    }
}

#[test]
fn enabled_disabled_and_fieldset_inheritance() {
    check(&[(":enabled", 25), (":disabled", 4), ("button:enabled", 2)]);
    check(&[("fieldset :disabled", 1), ("legend :enabled", 1)]);
}

#[test]
fn checkedness_defaults_and_radio_groups() {
    check(&[(":checked", 4), ("option:checked", 2), (":default", 4), (":indeterminate", 2)]);
}

#[test]
fn required_optional_and_editability() {
    check(&[(":required", 2), (":optional", 22), ("input:read-only", 10)]);
    check(&[("input:read-write", 8), (":read-write", 11), (":placeholder-shown", 1)]);
}

#[test]
fn static_constraint_validation() {
    /* email missing, email and url malformed, number over max, number off
     * its step, required select on its placeholder, and the form. */
    check(&[(":invalid", 7), (":valid", 14), ("input:valid", 10), ("form:invalid", 1)]);
    check(&[(":in-range", 2), (":out-of-range", 1)]);
}
