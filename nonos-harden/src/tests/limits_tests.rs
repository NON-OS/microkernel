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

//! A GO over a declared gap is never quiet: its own line counts the limits,
//! the JSON carries the count, and the text lists each one right under the
//! verdict line, which stays the one line a signature check reports.

use super::fake::Fake;
use super::fake_report::gap;
use super::sealed_fixture::Sealed;
use crate::exit;
use crate::plan::run;
use crate::sign::named::line;
use crate::verdict::{build, text, DEV_GO};

#[test]
fn a_go_over_a_declared_gap_counts_and_lists_it_under_the_verdict() {
    let (code, v) = Sealed::dev().verdict(Fake { report: gap, ..Fake::default() });
    let want = format!("{DEV_GO}, 1 declared limit");
    let got = (code, v["verdict"].as_str(), v["limits"].as_u64());
    assert_eq!(got, (exit::GO, Some(&*want), Some(1)), "{v:#}");
    let s = Sealed::dev();
    let mut ctx = s.ctx(Fake { report: gap, ..Fake::default() });
    let checks = run(&mut ctx);
    let text = text::render(&build(&ctx, checks, None));
    let after: Vec<&str> = text.lines().skip_while(|l| *l != want).collect();
    let listed = after.get(1).is_some_and(|l| {
        l.starts_with("  limit: verify.trust-chain: ") && l.contains("field asserts")
    });
    assert!(after.len() == 2 && listed, "{text}");
    assert_eq!(line(&text), want, "the signed one line");
}

#[test]
fn a_go_with_no_limits_reads_plain_and_counts_none() {
    let (code, v) = Sealed::release().verdict(Fake::default());
    assert_eq!(
        (code, v["verdict"].as_str(), v["limits"].as_u64()),
        (exit::GO, Some("GO"), Some(0))
    );
}
