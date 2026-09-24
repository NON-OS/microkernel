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

//! Argument parsing and the two commands. Exit codes: 0 done, 1 usage,
//! 2 no such disk, 3 the write failed, 4 the read-back failed.

mod confirm;
mod list;
mod receipt;
mod run;
mod source;
mod write;

pub fn run(args: &[String]) -> i32 {
    match args.first().map(String::as_str) {
        None | Some("list") => list::run(),
        Some("write") => {
            let Some(word) = args.get(1) else {
                eprintln!("install write <word> [--yes] [--reboot]");
                return 1;
            };
            let yes = args.iter().any(|a| a == "--yes");
            let reboot = args.iter().any(|a| a == "--reboot");
            write::run(word, yes, reboot)
        }
        Some(other) => {
            eprintln!("install: unknown command {other}");
            eprintln!("install [list] | install write <word> [--yes] [--reboot]");
            1
        }
    }
}
