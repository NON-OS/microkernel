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

//! `tree.build-inputs`: what each build the flake runs reads is what the
//! tree's sources say. tools/nix/inputs.py --check works out from the tree
//! every file each cargo build can read, and fails when the committed
//! tools/nix/inputs.json says otherwise (the flake's input-drift check).

use super::runner::Cmd;
use super::scratch::Scratch;
use crate::check::Check;
use crate::ctx::Ctx;
use std::time::Duration;

pub const ID: &str = "tree.build-inputs";
const LIMIT: Duration = Duration::from_secs(15 * 60);

pub fn check(ctx: &mut Ctx) -> Check {
    let Some(tree) = &ctx.tree else {
        return Check::not_run(ID, "no tree to check the build inputs of");
    };
    let scratch = match Scratch::new("inputs") {
        Ok(s) => s,
        Err(e) => return Check::not_run(ID, e),
    };
    let tool = tree.join("tools/nix/inputs.py").display().to_string();
    let args = vec![tool, "--check".to_string()];
    let cmd = Cmd { program: "python3".into(), args, cwd: tree.clone(), env: vec![], limit: LIMIT };
    match ctx.runner.run(&cmd, &scratch.0.join("run.log")) {
        Err(e) => Check::not_run(ID, e),
        Ok(d) if d.code == Some(0) => {
            Check::pass(ID, "tools/nix/inputs.json is what the tree's builds read")
        }
        Ok(d) => Check::fail(ID, format!("inputs.py --check exited {:?}: {}", d.code, d.tail)),
    }
}
