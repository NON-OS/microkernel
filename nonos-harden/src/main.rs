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

//! nonos-harden: one GO or NO-GO before an image touches a machine. It runs
//! the verifiers this tree holds over a sealed release and prints a single
//! verdict, every check with its result and reason. What GO means, and what
//! it does not, is in README.md and docs/release/nonos-harden.md.

mod check;
mod cli;
mod ctx;
mod deliver;
mod exit;
mod hash;
mod plan;
mod release;
mod sign;
mod tree;
mod usage;
mod verdict;
mod write;

#[cfg(test)]
mod tests;

use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let code = match cli::parse(&args) {
        Ok(cmd) => run(cmd),
        Err(e) => {
            eprintln!("nonos-harden: {e}\n\n{}", usage::SHORT);
            exit::USAGE
        }
    };
    ExitCode::from(code)
}

fn run(cmd: cli::Command) -> u8 {
    match cmd {
        cli::Command::Help => {
            print!("{}", usage::LONG);
            exit::GO
        }
        cli::Command::Version => {
            println!("nonos-harden {}", env!("CARGO_PKG_VERSION"));
            exit::GO
        }
        cli::Command::Check(args) => plan::check(&args),
        cli::Command::VerifySignature(args) => sign::verify::run(&args),
    }
}
