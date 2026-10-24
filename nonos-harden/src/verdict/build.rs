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

//! A verdict from the checks and what they found, the GO rule applied.

use super::{decide, Verdict, FORMAT, SCOPE};
use crate::check::Check;
use crate::ctx::Ctx;

pub fn build(ctx: &Ctx, checks: Vec<Check>, signer: Option<String>) -> Verdict {
    let (overall, verdict) = decide(ctx.kind, &checks);
    let tool = format!("nonos-harden {}", env!("CARGO_PKG_VERSION"));
    let release = ctx.release.display().to_string();
    let (record_sha256, tree_head) = (ctx.record_sha256.clone(), ctx.tree_head.clone());
    let tree = ctx.tree.as_ref().map(|t| t.display().to_string());
    let (format, scope, kind) = (FORMAT, SCOPE, ctx.kind);
    let limits = checks.iter().map(|c| c.limits.len()).sum();
    Verdict {
        format,
        tool,
        overall,
        verdict,
        kind,
        limits,
        release,
        record_sha256,
        tree,
        tree_head,
        signer,
        scope,
        checks,
    }
}
