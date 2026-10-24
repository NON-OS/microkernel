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

//! How a nonos-verify report becomes one check. Its exit status is not
//! enough, because it exits 0 when a check could not run (a gap): a check
//! that failed, or that has any status but pass, fails this one; one that
//! could not run (gap, skip) makes this one not run; a report with no checks
//! did not run. A gap the module states while every check passes is a limit
//! only if the tree's declared list names it; any other fails this check.

use super::declared::{Declared, FILE};
use super::report::{Entry, Report};
use crate::check::{named, Check};

const UNRUN: [&str; 2] = ["gap", "skip"];

pub fn judge(id: &'static str, sub: &str, r: &Report, code: Option<i32>, d: &Declared) -> Check {
    let line = |c: &Entry| format!("{} ({}): {}", c.name, c.status, c.detail);
    let is_unrun = |c: &&Entry| UNRUN.contains(&c.status.as_str());
    let unrun: Vec<String> = r.checks.iter().filter(is_unrun).map(line).collect();
    let failed: Vec<String> =
        r.checks.iter().filter(|c| c.status != "pass" && !is_unrun(c)).map(line).collect();
    if !failed.is_empty() {
        return Check::fail(id, named(failed));
    }
    if !unrun.is_empty() {
        let why = format!("parts of nonos-verify {sub} could not run: {}", named(unrun));
        return Check::not_run(id, why);
    }
    if r.checks.is_empty() {
        return Check::not_run(id, format!("nonos-verify {sub} ran no checks"));
    }
    if code != Some(0) {
        let why = format!("every check passed, yet nonos-verify {sub} exited {code:?}");
        return Check::fail(id, why);
    }
    let shown = |(w, n): &(String, String)| format!("not covered: {w} (it needs: {n})");
    let on_list = |(w, _): &&(String, String)| d.iter().any(|(m, dw)| m == sub && dw == w);
    let undeclared: Vec<String> = r.gaps.iter().filter(|g| !on_list(g)).map(shown).collect();
    if !undeclared.is_empty() {
        let why = format!(
            "nonos-verify {sub} states gaps {FILE} does not declare: {}",
            named(undeclared)
        );
        return Check::fail(id, why);
    }
    let n = r.checks.len();
    Check::pass(id, format!("all {n} checks of nonos-verify {sub} pass"))
        .with_limits(r.gaps.iter().map(shown).collect())
}
