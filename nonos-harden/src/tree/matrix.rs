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

//! `matrix.static-checks`, `matrix.proof-crates`: rows of the tree's verify
//! matrix (tools/nonos-verify-matrix): every static check the tree holds, and
//! every proof crate's tests. The matrix writes its evidence under the
//! directory it runs from, so it runs from a private one, and its summary
//! says which steps exited non-zero.

use super::runner::Cmd;
use super::scratch::Scratch;
use super::summary::{judge_row, steps};
use crate::check::Check;
use crate::ctx::Ctx;
use crate::release::read::read_regular;
use std::time::Duration;

pub const STATIC: &str = "matrix.static-checks";
pub const PROOFS: &str = "matrix.proof-crates";
const LABEL: &str = "nonos-harden";
const HOUR: u64 = 3600;

pub fn static_checks(ctx: &mut Ctx) -> Check {
    row(ctx, STATIC, "static-checks", Duration::from_secs(2 * HOUR))
}
pub fn proof_crates(ctx: &mut Ctx) -> Check {
    row(ctx, PROOFS, "proof-crates", Duration::from_secs(10 * HOUR))
}

fn row(ctx: &mut Ctx, id: &'static str, row: &str, limit: Duration) -> Check {
    let Some(tree) = &ctx.tree else {
        return Check::not_run(id, "no tree to run the verify matrix in");
    };
    let scratch = match Scratch::new(row) {
        Ok(s) => s,
        Err(e) => return Check::not_run(id, e),
    };
    let (tool, at) = (tree.join("tools/nonos-verify-matrix"), tree.display().to_string());
    let args =
        [&tool.display().to_string(), "--tree", &at, "--label", LABEL, row].map(|a| a.to_string());
    let cmd = Cmd {
        program: "python3".into(),
        args: args.to_vec(),
        cwd: scratch.0.clone(),
        env: vec![],
        limit,
    };
    let done = match ctx.runner.run(&cmd, &scratch.0.join("run.log")) {
        Ok(d) => d,
        Err(e) => return Check::not_run(id, e),
    };
    let summary = scratch.0.join("HANDOFF/evidence").join(LABEL).join("summary.json");
    match read_regular(&summary, 16 << 20).and_then(|b| steps(&b, row)) {
        Ok(steps) => judge_row(id, row, &steps, done.code),
        Err(e) => {
            Check::not_run(id, format!("the matrix left no summary of {row} ({e}): {}", done.tail))
        }
    }
}
