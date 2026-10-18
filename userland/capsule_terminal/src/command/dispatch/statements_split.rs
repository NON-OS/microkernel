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

//! A typed line cut into statements at `;`, `&&`, `||` and a lone `&`, quotes respected.

use alloc::vec::Vec;

use super::{joins_streams, push, Conn, Stmt};

pub fn split_program(line: &[u8]) -> Vec<Stmt<'_>> {
    let mut out: Vec<Stmt<'_>> = Vec::new();
    let (mut start, mut conn) = (0usize, Conn::Always);
    let (mut sq, mut dq) = (false, false);
    let mut i = 0;
    while i < line.len() {
        let c = line[i];
        if c == b'\'' && !dq {
            sq = !sq;
        } else if c == b'"' && !sq {
            dq = !dq;
        } else if !sq && !dq && c == b';' {
            push(&mut out, conn, &line[start..i], false);
            conn = Conn::Always;
            start = i + 1;
        } else if !sq && !dq && c == b'&' && line.get(i + 1) == Some(&b'&') {
            push(&mut out, conn, &line[start..i], false);
            conn = Conn::And;
            start = i + 2;
            i += 2;
            continue;
        } else if !sq && !dq && c == b'|' && line.get(i + 1) == Some(&b'|') {
            push(&mut out, conn, &line[start..i], false);
            conn = Conn::Or;
            start = i + 2;
            i += 2;
            continue;
        } else if !sq && !dq && c == b'&' && !joins_streams(line, i) {
            push(&mut out, conn, &line[start..i], true);
            conn = Conn::Always;
            start = i + 1;
        }
        i += 1;
    }
    push(&mut out, conn, &line[start..], false);
    out
}
