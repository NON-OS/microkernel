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

//! Which of the four the words ask for.

use crate::command::output::Output;

const USAGE: &[u8] =
    b"usage: market [list] | market info <id> | market install <id> | market uninstall <id>";

pub fn run(out: &mut Output<'_>, argv: &[&[u8]]) {
    match argv.get(1..).unwrap_or(&[]) {
        [] | [b"list"] => super::list::run(out),
        [b"info", id] => super::info::run(out, id),
        [b"install", id] => super::install::run(out, id),
        [b"uninstall", id] => super::uninstall::run(out, id),
        [b"help"] | [b"-h"] | [b"--help"] => usage(out),
        [b"info"] | [b"install"] | [b"uninstall"] => {
            out.writeln(b"market: which listing? `market list` shows every id");
            usage(out);
        }
        _ => {
            out.writeln(b"market: not a market command");
            usage(out);
        }
    }
}

fn usage(out: &mut Output<'_>) {
    out.writeln(USAGE);
    out.writeln(b"  list            every listing, whether it can install here, and if it has");
    out.writeln(b"  info <id>       publisher, version, description and each install gate");
    out.writeln(b"  install <id>    ask the system to install a Linux package listing");
    out.writeln(b"  uninstall <id>  ask the system to take away what installing it put here");
}
