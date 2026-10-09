// NONOS Operating System (AGPL-3.0-or-later)
//! Matching time is bounded three ways: a failed step never retries the
//! compounds to its left, one match stops after a fixed number of tests,
//! and a cascade stops author matching once it has spent its share. Each
//! test reads the process-wide step count, so they run one at a time.

use std::sync::Mutex;

use capsule_browser_proofs::browser::css::matching::{matches_selector, spent, Siblings};
use capsule_browser_proofs::browser::css::parse::parse_selectors;
use capsule_browser_proofs::browser::{css, dom};

static ONE_AT_A_TIME: Mutex<()> = Mutex::new(());

fn page(body: &str) -> dom::Dom {
    dom::parse(format!("<!DOCTYPE html><html><head></head><body>{body}</body></html>").as_bytes())
}

fn items(n: usize) -> dom::Dom {
    page(&format!("<ul>{}</ul>", "<li>x</li>".repeat(n)))
}

#[test]
fn a_failed_step_is_not_retried_higher_up() {
    let _one = ONE_AT_A_TIME.lock().unwrap_or_else(|e| e.into_inner());
    /* Twenty .a steps over 200 nested .a elements, and no .z anywhere: a
     * matcher that tried every placement of the .a steps would take
     * C(200, 20) tries. The walk has no ancestor filter, so this measures
     * the matcher alone. */
    let d = page(&format!("{}<span class=b>x</span>", "<div class=a>".repeat(200)));
    let leaf = css::select(&d, ".b", 1)[0];
    let sel = &parse_selectors(&format!(".z {}.b", ".a ".repeat(20)))[0];
    let before = spent();
    assert!(!matches_selector(&d, &Siblings::walk(), leaf, sel));
    assert!(spent() - before < 1_000, "spent {}", spent() - before);
}

#[test]
fn one_match_stops_after_a_fixed_number_of_tests() {
    let _one = ONE_AT_A_TIME.lock().unwrap_or_else(|e| e.into_inner());
    /* Each of 5,000 items scans every earlier sibling for .never, about
     * 12.5 million tests if nothing stopped it. */
    let d = items(5_000);
    let ul = css::select(&d, "ul", 1)[0];
    let sel = &parse_selectors("ul:has(.never ~ li)")[0];
    let before = spent();
    assert!(!matches_selector(&d, &Siblings::table(&d), ul, sel));
    let used = spent() - before;
    assert!((131_072..140_000).contains(&used), "spent {used}");
}

#[test]
fn a_cascade_stops_author_matching_once_its_steps_are_spent() {
    let _one = ONE_AT_A_TIME.lock().unwrap_or_else(|e| e.into_inner());
    /* Eight rules that each cost about 12.5 million tests over the list,
     * then one that colours every item. */
    let d = items(5_000);
    let sheet = format!("{}li{{color:#ff0000}}", ".never ~ li{color:#00ff00}".repeat(8));
    let (ul, lis) = (css::select(&d, "ul", 1)[0], css::select(&d, "li", usize::MAX));
    let before = spent();
    let st = css::compute(&d, &sheet);
    let used = spent() - before;
    assert!(used < 52_000_000, "spent {used}");
    let (first, last) = (st.styles[lis[0]].color, st.styles[lis[4_999]].color);
    assert_ne!(first, st.styles[ul].color, "early items are styled");
    assert_eq!(last, st.styles[ul].color, "late items keep the inherited colour");
}
