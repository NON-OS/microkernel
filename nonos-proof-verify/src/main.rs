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

//! `nonos-proof-verify`: the command line (`cli`) over the file system.

use nonos_proof_verify::{cli, read_file, Verdict};

fn main() {
    // An argument that is not UTF-8 names nothing this tool can check.
    let Ok(args) =
        std::env::args_os().skip(1).map(|a| a.into_string()).collect::<Result<Vec<_>, _>>()
    else {
        eprintln!("nonos-proof-verify: an argument is not UTF-8 (try --help)");
        std::process::exit(Verdict::CouldNotCheck.exit_code());
    };
    let out = cli(&args, &read_file);
    eprint!("{}", out.stderr);
    print!("{}", out.stdout);
    std::process::exit(out.code);
}
