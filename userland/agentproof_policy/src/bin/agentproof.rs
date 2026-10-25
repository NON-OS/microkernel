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

//! `agentproof`: reads the file named and prints what cli.rs answers.

use std::io::Read;
use std::process::ExitCode;

use agentproof_policy::cli::{answer, ask, unreadable, usage, Answer, Ask, USAGE};
use agentproof_policy::format::MAX_FILE;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let words: Vec<&str> = args.iter().map(String::as_str).collect();
    let a = match ask(&words) {
        Ask::Policy(verb, file) => read(file)
            .map_or_else(|e| unreadable(file, &e.to_string()), |text| answer(verb, file, &text)),
        Ask::Help => Answer { code: 0, out: vec![USAGE.into()], err: vec![] },
        Ask::Version => Answer {
            code: 0,
            out: vec![format!("agentproof {} (nonos-box 1)", env!("CARGO_PKG_VERSION"))],
            err: vec![],
        },
        Ask::Usage => usage(),
    };
    a.out.iter().for_each(|l| println!("{l}"));
    a.err.iter().for_each(|l| eprintln!("{l}"));
    ExitCode::from(a.code)
}

/// At most one byte past the largest file, so a longer one is refused as
/// too long rather than read whole.
fn read(file: &str) -> std::io::Result<Vec<u8>> {
    let mut text = Vec::new();
    std::fs::File::open(file)?.take(MAX_FILE as u64 + 1).read_to_end(&mut text)?;
    Ok(text)
}
