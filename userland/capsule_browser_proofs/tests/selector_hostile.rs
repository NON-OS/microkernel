// NONOS Operating System (AGPL-3.0-or-later)
//! Hostile selectors neither crash nor stall the engine. Each case once
//! aborted the capsule (panic=abort) or held it for seconds.

use capsule_browser_proofs::browser::css::matching::spent;
use capsule_browser_proofs::browser::{css, dom};

/* Run on a 2 MiB stack, the NONOS user stack size. */
fn small_stack<T: Send + 'static>(f: impl FnOnce() -> T + Send + 'static) -> T {
    let t = std::thread::Builder::new().stack_size(2 << 20).spawn(f).unwrap();
    t.join().unwrap()
}

#[test]
fn nth_child_arguments_near_i32_limits_do_not_overflow() {
    let d = dom::parse(b"<ul><li>a</li><li>b</li></ul>");
    /* i - b wrapped to i32::MIN and MIN % -1 trapped, from querySelectorAll
     * and from a single stylesheet rule alike. */
    assert_eq!(css::select(&d, "li:nth-child(-n-2147483647)", usize::MAX).len(), 0);
    assert_eq!(css::select(&d, "li:nth-child(-n+2147483647)", usize::MAX).len(), 2);
    assert_eq!(css::select(&d, "li:nth-child(99999999999999n-99999999999)", usize::MAX).len(), 0);
    assert_eq!(
        css::compute(&d, "li:nth-child(-n-2147483647){color:red}").styles.len(),
        d.nodes.len()
    );
}

#[test]
fn deep_nesting_is_refused_without_recursing_or_rescanning() {
    small_stack(|| {
        for (open, n) in
            [(":not(", 100_000), (":is(", 16_000), (":where(", 16_000), (":has(", 4_000)]
        {
            let s = format!("{}.a{}", open.repeat(n), ")".repeat(n));
            assert!(css::parse::parse_selectors(&s).is_empty(), "{open} x{n}");
        }
        /* 32 levels are fine, 33 are not. */
        let ok = format!("p{}.a{}", ":not(".repeat(32), ")".repeat(32));
        let deep = format!("p{}.a{}", ":not(".repeat(33), ")".repeat(33));
        assert!(!css::parse::parse_selectors(&ok).is_empty());
        assert!(css::parse::parse_selectors(&deep).is_empty());
        let d = dom::parse(b"<p class=a>x</p>");
        assert_eq!(css::select(&d, &ok, usize::MAX).len(), 1, "an even :not chain keeps .a");
    });
}

#[test]
fn a_selector_over_8_kib_is_invalid_and_lists_keep_1024() {
    let long = format!(".{}", "a".repeat(9000));
    assert!(css::parse::parse_selectors(&long).is_empty());
    let many: Vec<String> = (0..2000).map(|i| format!(".c{i}")).collect();
    assert_eq!(css::parse::parse_selectors(&many.join(",")).len(), 1024);
    let unbalanced = format!(":is({}", "(".repeat(100_000));
    assert!(css::parse::parse_selectors(&unbalanced).is_empty());
}

#[test]
fn ancestor_filter_answers_deep_universal_sheets_in_few_steps() {
    let mut h = String::from("<body>");
    h.push_str(&"<div>".repeat(380));
    h.push('x');
    let d = dom::parse(h.as_bytes());
    let one: Vec<String> = (0..64).map(|k| format!(".nomatch{k} {}*", "div ".repeat(29))).collect();
    let sheet = format!("{} {{color:red}}\n", one.join(",")).repeat(16);
    let before = spent();
    css::compute(&d, &sheet);
    /* 16 rules x 64 selectors x 384 elements, each rejected by the filter
     * before any ancestor is walked; it cost 540 ms of walks before. */
    assert!(spent() - before < 400_000, "spent {}", spent() - before);
}
