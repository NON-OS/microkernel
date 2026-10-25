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

//! Paths written as text: read from a string, and written for a package
//! element that names nothing yet.

use super::{Fault, FaultKind, Host, Machine};
use crate::grammar::{name_string, Cursor};
use crate::load::Aml;
use crate::namespace::Id;
use alloc::vec::Vec;

impl<H: Host> Machine<'_, H> {
    /// The object a path written as text names, from the current scope.
    pub(crate) fn resolve_text(&self, aml: &Aml, text: &[u8], at: u32) -> Result<Id, Fault> {
        let mut enc = Vec::with_capacity(text.len() + 2);
        let (prefix, rest) =
            text.split_at(text.iter().take_while(|c| matches!(c, b'\\' | b'^')).count());
        enc.extend_from_slice(prefix);
        let segs: Vec<&[u8]> =
            if rest.is_empty() { Vec::new() } else { rest.split(|c| *c == b'.').collect() };
        match segs.len() {
            0 => enc.push(0),
            1 => {}
            2 => enc.push(0x2E),
            n => enc.extend_from_slice(&[0x2F, n as u8]),
        }
        for s in &segs {
            let mut seg = [b'_'; 4];
            if s.is_empty() || s.len() > 4 {
                return Err(self.fault(FaultKind::Undefined, at));
            }
            seg[..s.len()].copy_from_slice(s);
            enc.extend_from_slice(&seg);
        }
        let bad = || self.fault(FaultKind::Undefined, at);
        let path = name_string(&mut Cursor::new(&enc, 0, enc.len() as u32)).map_err(|_| bad())?;
        aml.ns.resolve(self.frame().scope, &enc, path).ok_or_else(bad)
    }

    /// A NameString's text as ASL spells it, for a package element that
    /// names nothing yet.
    pub(crate) fn path_text(&self, aml: &Aml, t: u16, node: u32) -> Vec<u8> {
        let mut out = Vec::new();
        let Some(p) = aml.name_of(t, node) else { return out };
        let bytes = &aml.tables[t as usize].bytes;
        if p.root {
            out.push(b'\\');
        }
        out.resize(out.len() + p.up as usize, b'^');
        for i in 0..p.count {
            if i > 0 {
                out.push(b'.');
            }
            out.extend_from_slice(&p.seg(bytes, i));
        }
        out
    }
}
