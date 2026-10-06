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

//! Print the first lines of files (default ten, -n or -<count> to change).
//! With more than one file each is headed by its name; it used to read only
//! the last one named.

use alloc::vec::Vec;

use super::read_file::slurp;
use crate::command::dispatch::{lines_of, spec_for};
use crate::command::flags::{parse, parse_usize};
use crate::command::output::Output;
use crate::term::state::State;

/// The count and the files of a `head` or `tail`, or the line saying why not.
pub(super) fn ends_args<'a>(
    name: &[u8],
    argv: &[&'a [u8]],
) -> Result<(usize, Vec<&'a [u8]>), Vec<u8>> {
    let Some(spec) = spec_for(name) else { return Err(Vec::new()) };
    let parsed = parse(&spec, &argv[1..])?;
    let n = match parsed.value(b'n') {
        None => 10,
        Some(v) => parse_usize(v).ok_or_else(|| {
            let mut msg = Vec::from(name);
            msg.extend_from_slice(b": -n takes a count");
            msg
        })?,
    };
    if parsed.operands.is_empty() {
        let mut msg = Vec::from(name);
        msg.extend_from_slice(b": missing file");
        return Err(msg);
    }
    Ok((n, parsed.operands))
}

/// "==> name <==" over each file's lines when there is more than one.
pub(super) fn header(out: &mut Output<'_>, file: &[u8], first: bool) {
    if !first {
        out.writeln(b"");
    }
    let mut row = Vec::from(&b"==> "[..]);
    row.extend_from_slice(file);
    row.extend_from_slice(b" <==");
    out.writeln(&row);
}

pub fn head(state: &mut State, argv: &[&[u8]]) {
    let (n, files) = match ends_args(b"head", argv) {
        Ok(v) => v,
        Err(e) => return Output::new(&mut state.scrollback).writeln(&e),
    };
    for (i, file) in files.iter().enumerate() {
        let Some(bytes) = slurp(state, file) else { continue };
        let mut out = Output::new(&mut state.scrollback);
        if files.len() > 1 {
            header(&mut out, file, i == 0);
        }
        for line in lines_of(&bytes).iter().take(n) {
            out.writeln(line);
        }
    }
}
