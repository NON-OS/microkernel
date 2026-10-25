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

//! The parser's state and its two entry points.

use super::Scopes;
use crate::arena::{Arena, Span};
use crate::block::Block;
use crate::error::{Error, ErrorKind};
use crate::grammar::Cursor;

pub(super) struct Parser<'p, S: Scopes> {
    pub bytes: &'p [u8],
    pub arena: &'p mut Arena,
    pub scopes: &'p mut S,
    pub depth: u32,
}

/// Reads a definition block's term list into `arena` and returns the TERMS
/// node holding it. Method bodies stay DEFERRED; `body` reads them.
pub fn load<S: Scopes>(block: &Block<'_>, arena: &mut Arena, scopes: &mut S) -> Result<u32, Error> {
    if block.bytes.len() > arena.limits.max_table {
        return Err(Error::new(ErrorKind::Size, 4));
    }
    let (at, end) = block.aml();
    run(block.bytes, arena, scopes, Span { at, len: end - at })
}

/// Reads one DEFERRED method body. `scopes` must stand inside the method, as
/// its own TermList would see the namespace.
pub fn body<S: Scopes>(
    block: &Block<'_>,
    arena: &mut Arena,
    scopes: &mut S,
    span: Span,
) -> Result<u32, Error> {
    if span.end() > block.bytes.len() as u32 {
        return Err(Error::new(ErrorKind::PastParent, span.at));
    }
    run(block.bytes, arena, scopes, span)
}

/// A failed read leaves nothing behind: the arena goes back to where it was,
/// so a body that will not parse does not leave half an object for the
/// interpreter to find.
fn run<S: Scopes>(
    bytes: &[u8],
    arena: &mut Arena,
    scopes: &mut S,
    span: Span,
) -> Result<u32, Error> {
    let start = arena.checkpoint();
    let mut c = Cursor::new(bytes, span.at, span.end());
    let mut p = Parser { bytes, arena, scopes, depth: 0 };
    let r = p.terms(&mut c);
    if r.is_err() {
        p.arena.rollback(start);
    }
    r
}
