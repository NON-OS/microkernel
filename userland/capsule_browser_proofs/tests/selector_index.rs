// NONOS Operating System (AGPL-3.0-or-later)
//! A compound whose :is() or :where() arguments all name the same tag, id
//! or class carries that name itself, so the rule index files the rule
//! under it instead of testing it on every element. The name only restates
//! what the arguments require: matches and specificity stay the same.

use capsule_browser_proofs::browser::css::parse::parse_selectors;
use capsule_browser_proofs::browser::css::specificity::specificity;
use capsule_browser_proofs::browser::{css, dom};

type Key = (Option<String>, Option<String>, Vec<String>);

fn key(sel: &str) -> Key {
    let k = &parse_selectors(sel)[0].key;
    (k.tag.clone(), k.id.clone(), k.classes.clone())
}

fn names(tag: Option<&str>, id: Option<&str>, classes: &[&str]) -> Key {
    let owned = classes.iter().map(|c| c.to_string()).collect();
    (tag.map(Into::into), id.map(Into::into), owned)
}

#[test]
fn arguments_that_agree_on_a_name_key_the_compound() {
    assert_eq!(key(":is(.nav li):not(:last-child)"), names(Some("li"), None, &[]));
    assert_eq!(key(":where(.a .x.y, .b > .x)"), names(None, None, &["x"]));
    assert_eq!(key(":is(.a #m, .b > #m)"), names(None, Some("m"), &[]));
    assert_eq!(key(":is(:is(.a li))"), names(Some("li"), None, &[]), "through nesting");
    assert_eq!(key(":is(.a li, .b p)"), names(None, None, &[]), "no common name");
    assert_eq!(key("div:is(.a li)"), names(Some("div"), None, &[]), "a written tag stays");
    assert_eq!(key(":not(.a li)"), names(None, None, &[]), "a negation requires nothing");
    assert_eq!(key(":is()"), names(None, None, &[]));
}

#[test]
fn specificity_counts_what_was_written() {
    let spec = |s: &str| specificity(&parse_selectors(s)[0]);
    assert_eq!(spec(":where(.a .x.y, .b > .x)"), 0);
    assert_eq!(spec(":is(.nav li)"), (1 << 10) | 1);
    assert_eq!(spec(".k:is(.a #m)"), (1 << 20) | (2 << 10));
}

#[test]
fn keyed_rules_style_the_elements_they_match() {
    let d = dom::parse(
        b"<!DOCTYPE html><html><head></head><body><nav class=nav><ul><li>a</li><li>b</li></ul>\
</nav><ul><li>c</li><li>d</li></ul><p class='x y'>p</p><p class=x>q</p></body></html>",
    );
    let sheet = ":is(.nav li):not(:last-child){color:#ff0000} :where(p.y, .x.y){color:#00ff00}";
    let st = css::compute(&d, sheet);
    let body = css::select(&d, "body", 1)[0];
    let colour = |sel: &str| -> Vec<bool> {
        let ids = css::select(&d, sel, usize::MAX);
        ids.iter().map(|&i| st.styles[i].color != st.styles[body].color).collect()
    };
    assert_eq!(colour("li"), vec![true, false, false, false]);
    assert_eq!(colour("p"), vec![true, false]);
}
