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

//! The command line: each flag once and with a value, an unknown one refused,
//! so a typo can never run a weaker check than was asked for.

mod check;
mod flags;
mod verify;

use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Help,
    Version,
    Check(CheckArgs),
    VerifySignature(VerifyArgs),
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct CheckArgs {
    pub release: PathBuf,
    pub tree: Option<PathBuf>,
    pub out: Option<PathBuf>,
    pub sign_key: Option<PathBuf>,
    pub json: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub struct VerifyArgs {
    pub verdict: PathBuf,
    pub signature: PathBuf,
    pub public_key: String,
}

pub fn parse(args: &[String]) -> Result<Command, String> {
    match args.first().map(String::as_str) {
        None => Err("no command given".into()),
        Some("-h" | "--help" | "help") if args.len() == 1 => Ok(Command::Help),
        Some("-V" | "--version") if args.len() == 1 => Ok(Command::Version),
        Some("check") => check::check(&args[1..]).map(Command::Check),
        Some("verify-signature") => verify::verify(&args[1..]).map(Command::VerifySignature),
        Some(other) => Err(format!("unknown command or extra argument `{other}`")),
    }
}
