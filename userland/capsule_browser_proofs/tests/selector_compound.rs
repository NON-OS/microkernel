// NONOS Operating System (AGPL-3.0-or-later)
//! One compound, read in one pass: classes and ids after a pseudo-class
//! still count, :root composes with the rest instead of hiding it, escapes
//! decode, attribute tests stay inside :not() and :is(), and a :not()
//! argument keeps its case. Expected counts are Chromium's on this markup.

use capsule_browser_proofs::browser::{css, dom};

const PAGE: &str =
    "<!DOCTYPE html><html class=theme-light data-theme=light><head></head><body><ul>\
<li class='item first'><a href=/a class=rss_logo>A</a></li><li class=item><a class=x>B</a></li>\
<li class='item last' id=z>C</li></ul><button class='cdx-button quiet'>b</button>\
<button class='cdx-button quiet' disabled>d</button><input type=checkbox class=cb>\
<input type=text class=tx><div class='w-1/2 md:flex hover:bg-red !gl-absolute 2xl:p-4 a.b \
@sm/panel:gl-flex' id=x:y>tw</div><div class='Upper MixedCase'>u</div><div class=upper>l</div>\
</body></html>";

fn check(cases: &[(&str, usize)]) {
    let d = dom::parse(PAGE.as_bytes());
    for (sel, want) in cases {
        assert_eq!(css::select(&d, sel, usize::MAX).len(), *want, "{sel}");
    }
}

#[test]
fn classes_and_ids_after_a_pseudo_class_still_count() {
    check(&[
        ("a:link.rss_logo", 1),
        ("li:first-child.item", 1),
        ("li:last-child.last", 1),
        ("li:last-child#z", 1),
        (".cdx-button:enabled.quiet", 1),
    ]);
}

#[test]
fn root_composes_with_the_rest_of_its_compound() {
    check(&[
        (":root.theme-light", 1),
        (":root.theme-dark", 0),
        (":root.theme-dark li", 0),
        (":root:not(.theme-dark)", 1),
        (":root[data-theme=light]", 1),
        (":root[data-theme=dark]", 0),
        ("html:root", 1),
        ("li:root", 0),
    ]);
}

#[test]
fn escapes_decode_to_the_class_the_markup_carries() {
    check(&[
        (r".w-1\/2", 1),
        (r".md\:flex", 1),
        (r".\!gl-absolute", 1),
        (r".\32xl\:p-4", 1),
        (r".a\.b", 1),
        (r"#x\:y", 1),
        (r".\77-1\/2", 1),
        (r".\@sm\/panel\:gl-flex", 1),
    ]);
}

#[test]
fn attribute_tests_stay_inside_their_pseudo_class_and_keep_case() {
    check(&[
        ("a:not([href])", 1),
        ("input:not([type=checkbox])", 1),
        ("a:where(:not([role='button']))", 2),
        ("li:is([id])", 1),
        ("div:not(.Upper)", 2),
        ("div:is(.MixedCase)", 1),
    ]);
}
