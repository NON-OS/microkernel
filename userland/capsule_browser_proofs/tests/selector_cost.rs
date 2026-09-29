// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! A test pays for the bytes it reads, so thousands of selectors over one
//! huge attribute stop at the cascade's step budget, whatever its size.

use std::sync::Mutex;
use std::time::{Duration, Instant};

use capsule_browser_proofs::browser::css::matching::spent;
use capsule_browser_proofs::browser::{css, dom};

static ONE_AT_A_TIME: Mutex<()> = Mutex::new(());

/* MAX_MATCH_STEPS and CALL_STEPS: the cascade checks its share before
 * each candidate, so it overruns it by at most one call. */
const CASCADE: usize = 50_000_000;
const CALL: usize = 131_072;

/* One element whose attribute `attr` holds `value` (up to 1 MB, the most
 * a value may hold), under 3,000 rules that each read all of it and match
 * nothing, then one that colours it. Returns the steps and time spent. */
fn hostile(attr: &str, value: &str, sel: impl Fn(usize) -> String) -> (usize, Duration) {
    let _one = ONE_AT_A_TIME.lock().unwrap_or_else(|e| e.into_inner());
    let html = format!("<!DOCTYPE html><html><body><p><b {attr}='{value}'>x</b></p></body></html>");
    let d = dom::parse(html.as_bytes());
    let b = css::select(&d, "b", 1)[0];
    assert_eq!(d.nodes[b].attr(attr).map(str::len), Some(value.len()), "value kept whole");
    let rules: String = (0..3_000).map(|i| format!("{}{{color:#ff0000}}", sel(i))).collect();
    let sheet = format!("{rules}b{{color:#00ff00}}");
    let (before, start) = (spent(), Instant::now());
    let st = css::compute(&d, &sheet);
    let (used, took) = (spent() - before, start.elapsed());
    println!("{attr}: {used} steps in {took:?}");
    assert_ne!(st.styles[b].color, 0xff00_ff00, "the budget stopped author matching");
    (used, took)
}

fn bounded((used, took): (usize, Duration)) {
    assert!(used > CASCADE, "the budget was reached: {used}");
    assert!(used <= CASCADE + CALL + 1_000, "and overrun by at most one call: {used}");
    assert!(took < Duration::from_millis(1_500), "took {took:?}");
}

#[test]
fn substring_tests_over_a_1_mb_value() {
    let v = "a".repeat(1_000_000);
    bounded(hostile("data-x", &v, |i| format!("[data-x*=b{i}]")));
    bounded(hostile("data-x", &v, |i| format!("[data-x*=B{i} i]")));
}

#[test]
fn word_tests_over_a_1_mb_class() {
    bounded(hostile("class", &"a ".repeat(500_000), |i| format!("[class~=z{i}]")));
}

#[test]
fn lang_ranges_over_a_long_tag() {
    /* 250 KB: short enough that each call reads it before its steps run out. */
    let tag = format!("en{}", "-ab".repeat(83_332));
    bounded(hostile("lang", &tag, |i| format!(":lang(en-z{i})")));
}
