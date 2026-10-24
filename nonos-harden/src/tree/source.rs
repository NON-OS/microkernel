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
//! the receipt's commit or a later one, and the files on disk, hashed here
//! (`blobs`, `untracked`), differ from that commit only where the seal
//! stages what it writes (tools/nonos_seal: nonos-data/trust, market and
//! models). The kernel's public keys sit there but are pinned: a release's
//! must be the commit's, and a development image's own keys are a limit.

use super::{blobs::differences, git::git};
use crate::check::{named, Check};
use crate::ctx::{Ctx, Kind};
use crate::release::kernel_keys::{ED_PUB, PQ_PUB};

pub const ID: &str = "tree.source";
const STAGED: [&str; 3] = ["nonos-data/trust/", "nonos-data/market/", "nonos-data/models/"];

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
    let changed = match differences(tree, &c) {
        Ok(d) => d,
        Err(e) => return Check::fail(ID, format!("the tree could not be compared ({e})")),
    };
    let outside: Vec<String> =
        changed.iter().filter(|p| !STAGED.iter().any(|s| p.starts_with(s))).cloned().collect();
    if !outside.is_empty() {
        let n = outside.len();
        let why =
            format!("{n} files differ from {c} outside the seal's staging: {}", named(outside));
        return Check::fail(ID, why);
    }
    let keys = changed.iter().any(|p| p == ED_PUB || p == PQ_PUB);
    let (exact, staged) = (
        format!("exactly commit {c}, the release's source"),
        format!("commit {c}, changed only where the seal stages"),
    );
    match (keys, ctx.kind) {
        (true, Kind::Development) => {
            let own = format!(
                "the kernel's public keys are this development tree's own, not commit {c}'s"
            );
            Check::pass(ID, staged).with_limits(vec![own])
        }
        (true, _) => Check::fail(ID, format!("the kernel's public keys differ from commit {c}'s")),
        (false, _) => Check::pass(ID, if changed.is_empty() { exact } else { staged }),
    }
}
