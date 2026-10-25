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

//! The count and the files a `head` or `tail` was given.

use alloc::vec::Vec;

use crate::command::dispatch::spec_for;
use crate::command::flags::{parse, parse_usize};

/// The count and the files of a `head` or `tail`, or the line saying why not.
pub(in super::super) fn ends_args<'a>(
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
