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

//! `verify.hygiene`, `verify.trust-chain`, `verify.supply-chain`: the tree's
//! verification engine, nonos-verify, built from the tree and run on it, its
//! report written to a private directory and judged by `judge` against the
//! tree's declared limits, which are read first: without them none can pass.

use super::{declared, judge::judge, report::read, runner::Cmd, scratch::Scratch};
use crate::check::Check;
use crate::ctx::Ctx;
use std::path::Path;
use std::time::Duration;

pub const HYGIENE: &str = "verify.hygiene";
pub const TRUST_CHAIN: &str = "verify.trust-chain";
pub const SUPPLY_CHAIN: &str = "verify.supply-chain";
const LIMIT: Duration = Duration::from_secs(60 * 60);

pub fn hygiene(ctx: &mut Ctx) -> Check {
    run(ctx, HYGIENE, "hygiene")
}
pub fn trust_chain(ctx: &mut Ctx) -> Check {
    run(ctx, TRUST_CHAIN, "trust-chain")
}
pub fn supply_chain(ctx: &mut Ctx) -> Check {
    run(ctx, SUPPLY_CHAIN, "supply-chain")
}

fn run(ctx: &mut Ctx, id: &'static str, sub: &str) -> Check {
    let Some(tree) = &ctx.tree else {
        return Check::not_run(id, "no tree to run nonos-verify in");
    };
    let declared = match declared::load(tree) {
        Ok(d) => d,
        Err(e) => return Check::fail(id, e),
    };
    let scratch = match Scratch::new(sub) {
        Ok(s) => s,
        Err(e) => return Check::not_run(id, e),
    };
    let done = match ctx.runner.run(&command(tree, sub, &scratch.0), &scratch.0.join("run.log")) {
        Ok(d) => d,
        Err(e) => return Check::not_run(id, e),
    };
    let at = scratch.0.join(sub).join(format!("{sub}-report.json"));
    read(&at).map_or_else(
        |e| Check::not_run(id, format!("nonos-verify {sub} left no report ({e}): {}", done.tail)),
        |r| judge(id, sub, &r, done.code, &declared),
    )
}

fn command(tree: &Path, sub: &str, reports: &Path) -> Cmd {
    let manifest = tree.join("nonos-verify/Cargo.toml").display().to_string();
    let args = ["run", "--quiet", "--release", "--locked", "--manifest-path", &manifest, "--", sub];
    let env = vec![("NONOS_VERIFY_REPORTS".to_string(), reports.display().to_string())];
    let args = args.map(String::from).to_vec();
    Cmd { program: "cargo".into(), args, cwd: tree.into(), env, limit: LIMIT }
}
