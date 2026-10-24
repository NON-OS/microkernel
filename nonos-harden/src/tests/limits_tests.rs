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
use crate::release::sealed::UNBOUND;
use crate::sign::named::line;
use crate::verdict::{build, text, DEV_GO};

#[test]
fn a_go_over_a_declared_gap_counts_and_lists_it_under_the_verdict() {
    let (code, v) = Sealed::dev().verdict(Fake { report: gap, ..Fake::default() });
    let want = format!("{DEV_GO}, 2 declared limits");
    let got = (code, v["verdict"].as_str(), v["limits"].as_u64());
    assert_eq!(got, (exit::DEV_GO, Some(&*want), Some(2)), "{v:#}");
    let s = Sealed::dev();
    let mut ctx = s.ctx(Fake { report: gap, ..Fake::default() });
    let checks = run(&mut ctx);
    let text = text::render(&build(&ctx, checks, None));
    let after: Vec<&str> = text.lines().skip_while(|l| *l != want).collect();
    let listed = |id: &str, what: &str| {
        after.iter().any(|l| l.starts_with(&format!("  limit: {id}: ")) && l.contains(what))
    };
    let both = listed("verify.trust-chain", "field asserts")
        && listed("release.sealed", "unsigned release record");
    assert!(after.len() == 3 && both, "{text}");
    assert_eq!(line(&text), want, "the signed one line");
}

#[test]
fn what_the_unsigned_record_alone_holds_is_a_limit_on_every_go() {
    let (code, v) = Sealed::release().verdict(Fake::default());
    let got = (code, v["verdict"].as_str(), v["limits"].as_u64());
    assert_eq!(got, (exit::GO, Some("GO, 1 declared limit"), Some(1)), "{v:#}");
    let sealed = v["checks"].as_array().unwrap().iter().find(|c| c["id"] == "release.sealed");
    assert_eq!(sealed.unwrap()["limits"][0].as_str(), Some(UNBOUND), "{v:#}");
}
