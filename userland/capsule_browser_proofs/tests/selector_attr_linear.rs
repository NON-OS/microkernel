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

use std::hint::black_box;
use std::time::{Duration, Instant};

use capsule_browser_proofs::browser::css::selector::{AttrOp, AttrTest};

fn test(value: &str, ci: bool) -> AttrTest {
    AttrTest { op: AttrOp::Contains, value: value.into(), case_insensitive: ci }
}

/* The best of five: a pause the machine takes elsewhere counts for neither. */
fn best(mut run: impl FnMut()) -> Duration {
    (0..5)
        .map(|_| {
            let start = Instant::now();
            run();
            start.elapsed()
        })
        .min()
        .unwrap_or_default()
}

/* 1 MB of 'a' (the longest value an attribute keeps) against 8,000 'a'
 * then 'b' (a value the 8 KiB selector cap still admits): a window by
 * window search compares 8,001 bytes at each of 992,000 offsets, about a
 * thousand times one pass over the attribute. The search is core's, so its
 * steps cannot be counted here; it is timed against one pass over the same
 * megabyte in the same run instead, which a loaded machine slows alike. */
#[test]
fn a_near_miss_at_every_offset_stays_linear() {
    let have = "a".repeat(1_000_000);
    let value = format!("{}b", "a".repeat(8_000));
    let pass = best(|| _ = black_box(black_box(&have).bytes().filter(|b| *b == b'b').count()));
    for ci in [false, true] {
        let t = test(&value, ci);
        let took = best(|| assert!(!t.matches(black_box(&have))));
        println!("[attr*=v] near miss over 1 MB, i {ci}: {took:?}, one pass {pass:?}");
        let bound = pass * 50 + Duration::from_millis(2);
        assert!(took < bound, "i {ci}: {took:?} against {bound:?}");
    }
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
