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


//! `log`: the last of what the kernel wrote to its serial console, read back
//! with `mk_log_tail`, for a machine with no serial port. `log` alone shows
//! the newest lines; `log rtl tpm` only the lines naming any word given.
//! `log > f` keeps them in a file like any command's output.

use alloc::vec;

use nonos_libc::mk_log_tail;

use crate::command::output::Output;

/* The kernel keeps 64 KiB; the newest lines shown when no word is given. */
const KEPT: usize = 128 * 1024;
const NEWEST: usize = 200;

pub fn run(out: &mut Output<'_>, argv: &[&[u8]]) {
    let words = &argv[1.min(argv.len())..];
    let mut buf = vec![0u8; KEPT];
    let rc = mk_log_tail(&mut buf);
    if rc < 0 {
        out.writeln_error(b"log: the kernel would not hand over its log (this terminal needs AttestRead)");
        return;
    }
    let text = &buf[..rc as usize];
    let lines: alloc::vec::Vec<&[u8]> = text
        .split(|&b| b == b'\n')
        .map(|l| l.strip_suffix(b"\r").unwrap_or(l))
        .filter(|l| !l.is_empty())
        .filter(|l| words.is_empty() || words.iter().any(|w| contains(l, w)))
        .collect();
    let from = if words.is_empty() { lines.len().saturating_sub(NEWEST) } else { 0 };
    if lines.is_empty() {
        out.writeln(b"log: no line matches");
    }
    for line in &lines[from..] {
        out.writeln(line);
    }
}

fn contains(hay: &[u8], needle: &[u8]) -> bool {
    !needle.is_empty() && hay.windows(needle.len()).any(|w| w.eq_ignore_ascii_case(needle))
}
