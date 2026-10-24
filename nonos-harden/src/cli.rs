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

//! The command line: each flag once, with a value; an unknown one is refused.

use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Help,
    Version,
    Check(CheckArgs),
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct CheckArgs {
    pub release: PathBuf,
    pub tree: Option<PathBuf>,
    pub out: Option<PathBuf>,
    pub json: bool,
}

const CHECK_FLAGS: &[&str] = &["--release", "--tree", "--out", "--format"];

pub fn parse(args: &[String]) -> Result<Command, String> {
    match args.first().map(String::as_str) {
        None => Err("no command given".into()),
        Some("-h" | "--help" | "help") if args.len() == 1 => Ok(Command::Help),
        Some("-V" | "--version") if args.len() == 1 => Ok(Command::Version),
        Some("check") => check(&args[1..]).map(Command::Check),
        Some(other) => Err(format!("unknown command or extra argument `{other}`")),
    }
}

fn check(args: &[String]) -> Result<CheckArgs, String> {
    let given = flags(args, CHECK_FLAGS)?;
    let get = |name: &str| given.iter().find(|(f, _)| *f == name).map(|(_, v)| v.as_str());
    let release = get("--release").ok_or("`--release DIR` is required")?;
    let json = match get("--format") {
        None | Some("text") => false,
        Some("json") => true,
        Some(f) => return Err(format!("`--format` is `text` or `json`, not `{f}`")),
    };
    let (tree, out) = (get("--tree").map(PathBuf::from), get("--out").map(PathBuf::from));
    Ok(CheckArgs { release: release.into(), tree, out, json })
}

/// `--flag value` pairs, each flag known and given once, each value present.
pub fn flags<'a>(args: &[String], known: &[&'a str]) -> Result<Vec<(&'a str, String)>, String> {
    let mut given: Vec<(&str, String)> = Vec::new();
    let mut it = args.iter();
    while let Some(flag) = it.next() {
        let name = *known.iter().find(|k| *k == flag).ok_or(format!("unknown flag `{flag}`"))?;
        let value = it.next().filter(|v| !v.is_empty() && !v.starts_with("--"));
        let value = value.ok_or(format!("`{name}` needs a value"))?;
        if given.iter().any(|(n, _)| *n == name) {
            return Err(format!("`{name}` given twice"));
        }
        given.push((name, value.clone()));
    }
    Ok(given)
}
