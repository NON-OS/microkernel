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

//! `tree.source`: the tree is the source the release was built from. HEAD is
//! the receipt's commit or a later one, and the files (tracked or not) differ
//! from that commit only in the trust set under nonos-data/: the seal stages
//! its trust set after the commit, so that is the one change a sealing tree
//! carries. Anything else changed means the tree's verifiers would judge
//! other source than the image's.

use super::git::{git, paths};
use crate::check::Check;
use crate::ctx::Ctx;

pub const ID: &str = "tree.source";
const TRUST_SET: &str = "nonos-data/";
const NAMED: usize = 5;

pub fn check(ctx: &mut Ctx) -> Check {
    let (Some(tree), Some(r)) = (&ctx.tree, &ctx.receipt) else {
        return Check::not_run(ID, "needs the tree and the build receipt's commit");
    };
    let c = r.commit.clone();
    let head = match git(tree, &["rev-parse", "HEAD"]) {
        Ok(h) => h.trim().to_string(),
        Err(e) => return Check::fail(ID, format!("the tree is not a git checkout ({e})")),
    };
    ctx.tree_head = Some(head.clone());
    if git(tree, &["cat-file", "-e", &format!("{c}^{{commit}}")]).is_err() {
        return Check::fail(ID, format!("the tree does not hold commit {c}, the release's source"));
    }
    if git(tree, &["merge-base", "--is-ancestor", &c, "HEAD"]).is_err() {
        return Check::fail(ID, format!("HEAD {head} is neither commit {c} nor a later one"));
    }
    let changed = git(tree, &["diff", "--name-only", "-z", &c, "--"]);
    let untracked = git(tree, &["ls-files", "-z", "--others", "--exclude-standard"]);
    let (changed, untracked) = match (changed, untracked) {
        (Ok(a), Ok(b)) => (a, b),
        (Err(e), _) | (_, Err(e)) => {
            return Check::fail(ID, format!("git could not compare ({e})"))
        }
    };
    let all: Vec<&str> = paths(&changed).chain(paths(&untracked)).collect();
    let outside: Vec<&str> = all.iter().copied().filter(|p| !p.starts_with(TRUST_SET)).collect();
    if !outside.is_empty() {
        let named = outside.iter().take(NAMED).copied().collect::<Vec<_>>().join(", ");
        let n = outside.len();
        return Check::fail(
            ID,
            format!("{n} files differ from commit {c} outside the trust set: {named}"),
        );
    }
    match all.is_empty() && head == c {
        true => Check::pass(ID, format!("exactly commit {c}, the release's source")),
        false => Check::pass(
            ID,
            format!("commit {c}, the release's source, changed only in its trust set"),
        ),
    }
}
