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

//! The verdict as people read it: one row per check, then the one line, and
//! right under it every limit a passing check carries.
//! Every string that came from outside this program (a reason quoting a file
//! name, a verifier's report) is printed with control and direction-override
//! characters replaced, so nothing in a release can steer the terminal or
//! reorder the text of the verdict that judges it.

use super::Verdict;
use crate::check::Status;

pub fn render(v: &Verdict) -> String {
    let mut s = format!("{} ({})\nrelease: {}\n", v.tool, v.format, clean(&v.release));
    if let Some(sha) = &v.record_sha256 {
        s += &format!("record sha256: {sha}\n");
    }
    match (&v.tree, &v.tree_head) {
        (Some(t), Some(h)) => s += &format!("tree: {} at {h}\n", clean(t)),
        (Some(t), None) => s += &format!("tree: {}\n", clean(t)),
        _ => s += "tree: none found; give --tree\n",
    }
    let w = v.checks.iter().map(|c| c.id.len()).max().unwrap_or(0);
    for c in &v.checks {
        let tag = match c.status {
            Status::Pass => "pass",
            Status::Fail => "FAIL",
            Status::NotRun => "NOT RUN",
        };
        s += &format!("  {tag:<7}  {:<w$}  {}\n", c.id, clean(&c.reason));
    }
    if let Some(k) = &v.signer {
        s += &format!("signer: {k}\n");
    }
    s += &format!("scope: {}\n{}\n", v.scope, v.verdict);
    for c in &v.checks {
        for l in &c.limits {
            s += &format!("  limit: {}: {}\n", c.id, clean(l));
        }
    }
    s
}

pub fn clean(s: &str) -> String {
    let unsafe_char = |c: char| {
        c.is_control()
            || matches!(c, '\u{200E}' | '\u{200F}' | '\u{202A}'..='\u{202E}' | '\u{2066}'..='\u{2069}')
    };
    s.chars().map(|c| if unsafe_char(c) { '\u{FFFD}' } else { c }).collect()
}
