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

//! [attr*=v] finds its value in time linear in the attribute and value,
//! with or without the i flag: a value that almost matches at every offset
//! costs no more than one that never starts to.

use std::time::{Duration, Instant};

use capsule_browser_proofs::browser::css::selector::{AttrOp, AttrTest};

fn test(value: &str, ci: bool) -> AttrTest {
    AttrTest { op: AttrOp::Contains, value: value.into(), case_insensitive: ci }
}

/* 1 MB of 'a' (the longest value an attribute keeps) against 8,000 'a'
 * then 'b' (a value the 8 KiB selector cap still admits): a window by
 * window search compares 8,001 bytes at each of 992,000 offsets. */
fn timed(ci: bool) -> Duration {
    let have = "a".repeat(1_000_000);
    let t = test(&format!("{}b", "a".repeat(8_000)), ci);
    let start = Instant::now();
    assert!(!t.matches(&have));
    start.elapsed()
}

#[test]
fn a_near_miss_at_every_offset_stays_linear() {
    let took = [timed(false), timed(true)];
    println!("[attr*=v] near miss over 1 MB: {:?}, with i: {:?}", took[0], took[1]);
    assert!(took.iter().all(|t| *t < Duration::from_millis(20)), "took {took:?}");
}

#[test]
fn contains_still_answers_as_before() {
    let have = format!("{}Needle{}", "x".repeat(1_000), "y".repeat(1_000));
    assert!(test("Needle", false).matches(&have));
    assert!(!test("needle", false).matches(&have));
    assert!(test("nEEDLE", true).matches(&have));
    assert!(test("XNEEDLEY", true).matches(&have));
    assert!(!test("", false).matches(&have), "an empty value never matches");
    assert!(!test("", true).matches(&have));
    assert!(!test("xNeedlez", true).matches(&have));
    assert!(test("\u{e9}t\u{e9}", true).matches("l'\u{e9}t\u{e9}"), "non-ASCII compares exactly");
    assert!(!test("\u{c9}T\u{c9}", true).matches("l'\u{e9}t\u{e9}"), "only ASCII folds");
}
