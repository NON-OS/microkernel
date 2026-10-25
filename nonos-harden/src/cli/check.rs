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

//! `check`'s flags. `--release` is required; `--sign-key` signs the verdict
//! file, so it needs `--out`.

use super::flags::Given;
use super::CheckArgs;
use std::path::PathBuf;

const FLAGS: &[&str] = &["--release", "--tree", "--out", "--format", "--sign-key"];

pub fn check(args: &[String]) -> Result<CheckArgs, String> {
    let given = Given::read(args, FLAGS)?;
    let release = given.get("--release").ok_or("`--release DIR` is required")?;
    let json = match given.get("--format") {
        None | Some("text") => false,
        Some("json") => true,
        Some(f) => return Err(format!("`--format` is `text` or `json`, not `{f}`")),
    };
    let path = |name: &str| given.get(name).map(PathBuf::from);
    let (tree, out, sign_key) = (path("--tree"), path("--out"), path("--sign-key"));
    if sign_key.is_some() && out.is_none() {
        return Err("`--sign-key` signs the verdict file, so it needs `--out FILE`".into());
    }
    Ok(CheckArgs { release: release.into(), tree, out, sign_key, json })
}
