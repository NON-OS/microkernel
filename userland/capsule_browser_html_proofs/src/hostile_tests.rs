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

//! Input built to exhaust the parser: deep nesting, unmatched end tags and
//! misnested formatting each stay bounded and leave a well-formed tree.

use std::time::Instant;

use crate::browser::dom::limits::MAX_DEPTH;
use crate::shape::{all, check, depth, doc};

#[test]
fn nesting_stops_at_the_depth_cap_and_keeps_every_element() {
    let d = doc(&("<div>".repeat(10_000) + "end"));
    check(&d);
    assert!(depth(&d) <= MAX_DEPTH + 1, "depth {}", depth(&d));
    assert_eq!(all(&d, "div").len(), 10_000);
    assert!(d.nodes.iter().any(|n| n.text == "end"));
}

#[test]
fn unmatched_end_tags_under_a_deep_stack_cost_constant_time() {
    let html = "<div>".repeat(300) + &"</span>".repeat(300_000) + "x";
    let start = Instant::now();
    let d = doc(&html);
    let took = start.elapsed().as_millis();
    check(&d);
    assert!(took < 2_000, "300,000 unmatched end tags took {took} ms");
}

#[test]
fn misnested_formatting_floods_stay_bounded() {
    let html = "<b><i>".repeat(20_000) + "<p>x" + &"</b></i>".repeat(20_000);
    let start = Instant::now();
    let d = doc(&html);
    let took = start.elapsed().as_millis();
    check(&d);
    assert!(depth(&d) <= MAX_DEPTH + 1);
    assert!(took < 2_000, "formatting flood took {took} ms");
}
