// NONOS Operating System (AGPL-3.0-or-later)
//! Parse budgets: nesting that multiplies selector text, 2 MiB of long
//! selector lists, and more @layer names than ranks fit in, each parsed
//! in bounded time and within the page's rule budget.

use alloc::format;
use alloc::string::String;
use std::time::Instant;

use crate::browser::css::parse::parse;

use super::probe::Styles;

/* (style rules, selectors, estimated bytes) of a parsed sheet. */
fn kept(css: &str) -> (usize, usize, usize) {
    let rules = parse(css);
    let style = rules.iter().filter(|r| !r.selectors.is_empty()).count();
    let sels = rules.iter().map(|r| r.selectors.len()).sum();
    (style, sels, rules.iter().map(|r| r.cost as usize).sum())
}

#[test]
fn nesting_that_quadruples_each_level_stops_at_the_byte_bound() {
    let mut css = String::from(".abcdefgh{color:red;");
    css.push_str(&"& & & &{color:red;".repeat(14));
    css.push_str(&"}".repeat(15));
    let t = Instant::now();
    let (style, sels, bytes) = kept(&css);
    assert!(t.elapsed().as_millis() < 2_000, "{} ms", t.elapsed().as_millis());
    /* Level k's selector has 4^k compounds: levels 0 to 3 stay within the
     * 64 compounds a selector may chain, so level 4 and all under it drop
     * before the 4 KiB bound on a joined selector is reached. */
    assert_eq!((style, sels), (4, 4));
    assert!(bytes < 64 << 10, "{bytes} bytes");
}

#[test]
fn a_two_mib_sheet_of_selector_lists_stays_within_the_budget() {
    let list = [".a#b[c] > d:not(.e) ~ f"; 256].join(",");
    let rule = format!("{list}{{color:red}}");
    let css = rule.repeat((2 << 20) / rule.len());
    let t = Instant::now();
    let (style, sels, bytes) = kept(&css);
    assert!(t.elapsed().as_millis() < 5_000, "{} ms", t.elapsed().as_millis());
    /* Rule::MAX_SELECTORS and Rule::MAX_BYTES, which css keeps private. */
    assert!(sels <= 65_536 && bytes <= 16 << 20, "{sels} {bytes}");
    assert!(style < css.len() / rule.len(), "the budget stopped the parse: {style} rules");
}

#[test]
fn seventy_thousand_layers_keep_their_order() {
    let names: String = (0..70_000).map(|i| format!("l{i},")).collect();
    let css = format!(
        "#y{{color:#00ff00}}@layer base{{#x,#z{{color:#ff0000!important}}#y{{color:#ff0000}}}}\
         @layer {}top;@layer top{{#x{{color:#0000ff}}#y{{color:#0000ff}}#z{{color:#0000ff!important}}}}\
         #x{{color:#00ff00}}",
        names
    );
    let s = Styles::of(&format!("<style>{css}</style><p id=x>x</p><p id=y>y</p><p id=z>z</p>"));
    assert_eq!(s.get("y").color, 0xff00_ff00, "a layer named past the cap is still a layer");
    assert_eq!(s.get("x").color, 0xffff_0000, "important in base beats everything");
    assert_eq!(s.get("z").color, 0xffff_0000, "important reverses: base beats top");
}
