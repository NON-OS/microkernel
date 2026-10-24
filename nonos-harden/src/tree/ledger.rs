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

//! `tree.ledger`: the trust set in the tree is the one its ledger names. The
//! seal stamps nonos-data/trust/MANIFEST.sha256 over every file under the
//! trust set's capsules/, keys/ and policy/; each must have its digest, and
//! no file there may be missing from the ledger or added beside it.

use super::manifest::{parse, MAX_BYTES};
use crate::check::{named, Check};
use crate::ctx::Ctx;
use crate::release::{read::read_regular, resolve::resolve, sealed::digest, walk::files};
use std::collections::BTreeSet;

pub const ID: &str = "tree.ledger";
pub const TRUST: &str = "nonos-data/trust";
const PARTS: [&str; 3] = ["capsules", "keys", "policy"];
const MAX_FILES: usize = 100_000;

pub fn check(ctx: &mut Ctx) -> Check {
    let Some(tree) = &ctx.tree else {
        return Check::not_run(ID, "no tree to read the trust ledger from");
    };
    let trust = tree.join(TRUST);
    let entries =
        match read_regular(&trust.join("MANIFEST.sha256"), MAX_BYTES).and_then(|b| parse(&b)) {
            Ok(e) => e,
            Err(e) => return Check::fail(ID, format!("{TRUST}/MANIFEST.sha256: {e}")),
        };
    let mut bad = Vec::new();
    for (sha, rel) in &entries {
        match PARTS.iter().any(|p| rel.starts_with(&format!("{p}/"))) {
            false => bad.push(format!("the ledger names {rel}, outside the trust set's parts")),
            true => match resolve(&trust, rel, false) {
                Ok(at) => digest(&at, sha, rel, &mut bad),
                Err(e) => bad.push(e),
            },
        }
    }
    let listed: BTreeSet<&str> = entries.iter().map(|(_, r)| r.as_str()).collect();
    for p in PARTS {
        match files(&trust.join(p), MAX_FILES) {
            Ok(found) => bad.extend(
                found
                    .iter()
                    .map(|f| format!("{p}/{f}"))
                    .filter(|f| !listed.contains(f.as_str()))
                    .map(|f| format!("{f} is in the trust set but not in its ledger")),
            ),
            Err(e) => bad.push(format!("{p}: {e}")),
        }
    }
    match bad.is_empty() {
        true => Check::pass(
            ID,
            format!("all {} files of the trust set match its ledger", entries.len()),
        ),
        false => Check::fail(ID, named(bad)),
    }
}
