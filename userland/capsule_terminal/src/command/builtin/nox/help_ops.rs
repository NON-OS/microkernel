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

//! The nox index's closing rows: redirects, filters after a pipe, and joined commands.

use crate::command::output::Output;

/// The rows after the commands: what `>`, `|`, `;`, `&&` and `||` do.
pub(super) fn operators(out: &mut Output<'_>) {
    out.writeln(b"");
    out.writeln(b"  any command > file    write output to a file");
    out.writeln(b"  any command >> file   append output to a file");
    out.writeln(b"  cmd | grep [-cinv] p  keep (-v drop) matching lines; -n number, -c count");
    out.writeln(b"  cmd | sort            sort lines");
    out.writeln(b"  cmd | uniq            drop repeated adjacent lines");
    out.writeln(b"  cmd | nl              number lines");
    out.writeln(b"  cmd | wc [-l -w -c]   count lines, words and bytes");
    out.writeln(b"  cmd | head [-n] [n]   first n lines (default 10)");
    out.writeln(b"  cmd | tail [-n] [n]   last n lines (default 10)");
    out.writeln(b"  a ; b                 run a then b");
    out.writeln(b"  a && b                run b only if a succeeds");
    out.writeln(b"  a || b                run b only if a fails");
    out.writeln(b"");
    out.writeln(b"  Tab completes commands and paths.");
}
