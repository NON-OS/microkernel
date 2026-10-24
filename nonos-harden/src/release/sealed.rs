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

//! `release.sealed`: every artifact the record names has the bytes it names.
//! A file is hashed whole; a sealed directory must hold exactly the files the
//! record lists, each with its digest, no more and no fewer. One flipped byte
//! anywhere in what was sealed fails this check, and the reason names where.

use super::record::{Item, Sealed};
use super::sealed_dir::compare;
use super::walk::resolve;
use crate::check::Check;
use crate::ctx::Ctx;
use crate::hash::{sha256_file, short};
use std::path::Path;

pub const ID: &str = "release.sealed";
/// A reason names at most this many differences, then says how many more.
const NAMED: usize = 8;

pub fn check(ctx: &mut Ctx) -> Check {
    let Some(rec) = &ctx.record else {
        return Check::not_run(ID, "no release record to compare against");
    };
    let mut bad = Vec::new();
    for s in &rec.sealed {
        one(&ctx.release, s, &mut bad);
    }
    let n = rec.sealed.len();
    if bad.is_empty() {
        return Check::pass(ID, format!("all {n} sealed artifacts have the bytes the record names"));
    }
    let more = bad.len().saturating_sub(NAMED);
    let mut reason = bad.into_iter().take(NAMED).collect::<Vec<_>>().join("; ");
    if more > 0 {
        reason += &format!("; and {more} more");
    }
    Check::fail(ID, reason)
}

fn one(root: &Path, s: &Sealed, bad: &mut Vec<String>) {
    let failed = match &s.item {
        Item::File(want) => resolve(root, &s.rel, false).map(|at| digest(&at, want, &s.rel, bad)),
        Item::Dir(want) => compare(root, &s.rel, want, bad),
    };
    if let Err(e) = failed {
        bad.push(format!("{}: {e}", s.name));
    }
}

pub(super) fn digest(at: &Path, want: &str, shown: &str, bad: &mut Vec<String>) {
    match sha256_file(at) {
        Ok(got) if got == want => {}
        Ok(got) => bad.push(format!("{shown}: sha256 {}… is not the sealed {}…", short(&got), short(want))),
        Err(e) => bad.push(format!("{shown}: unreadable ({e})")),
    }
}
