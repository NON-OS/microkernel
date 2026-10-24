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
//! report written to a private directory and judged by `judge`.

use super::judge::judge;
use super::report::read;
use super::runner::Cmd;
use super::scratch::Scratch;
use crate::check::Check;
use crate::ctx::Ctx;
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
    let scratch = match Scratch::new(sub) {
        Ok(s) => s,
        Err(e) => return Check::not_run(id, e),
    };
    let manifest = tree.join("nonos-verify/Cargo.toml").display().to_string();
    let args = ["run", "--quiet", "--release", "--manifest-path", &manifest, "--", sub];
    let env = vec![("NONOS_VERIFY_REPORTS".to_string(), scratch.0.display().to_string())];
    let cmd = Cmd {
        program: "cargo".into(),
        args: args.map(String::from).to_vec(),
        cwd: tree.clone(),
        env,
        limit: LIMIT,
    };
    let done = match ctx.runner.run(&cmd, &scratch.0.join("run.log")) {
        Ok(d) => d,
        Err(e) => return Check::not_run(id, e),
    };
    match read(&scratch.0.join(sub).join(format!("{sub}-report.json"))) {
        Ok(r) => judge(id, sub, &r, done.code),
        Err(e) => {
            Check::not_run(id, format!("nonos-verify {sub} left no report ({e}): {}", done.tail))
        }
    }
}
