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

//! LoadTable (section 19.6.75): a table of the platform's, found by
//! signature and OEM ids, loaded under RootPathString, with ParameterData
//! stored to ParameterPathString once it is in.

use crate::exec::{Fault, FaultKind, Host, Machine};
use crate::load::Aml;
use crate::namespace::ROOT;
use crate::object::Object;
use alloc::vec::Vec;

impl<H: Host> Machine<'_, H> {
    /// The DDBHandle, or Zero if no such table exists or it was refused.
    pub(crate) fn load_table_op(
        &mut self,
        aml: &mut Aml,
        t: u16,
        node: u32,
        at: u32,
    ) -> Result<Object, Fault> {
        let k = aml.kids(t, node);
        let mut text: [Vec<u8>; 5] = Default::default();
        for (i, slot) in text.iter_mut().enumerate() {
            match self.value(aml, t, k[i])? {
                Object::String(s) => *slot = s.borrow().clone(),
                _ => return Err(self.fault(FaultKind::Type, at)),
            }
        }
        let Some(bytes) = self.host.table(&text[0], &text[1], &text[2]) else {
            return Ok(Object::Integer(0));
        };
        if !self.host.admit_table(&bytes) {
            return Ok(Object::Integer(0));
        }
        let scope = match text[3].as_slice() {
            b"" | b"\\" => ROOT,
            path => self.resolve_text(aml, path, at)?,
        };
        let Ok(index) = aml.load_table_at(bytes, scope, self) else {
            return Ok(Object::Integer(0));
        };
        self.announce_loaded(aml);
        if !text[4].is_empty() {
            let data = self.value(aml, t, k[5])?;
            let saved = self.frames.last().map(|f| f.scope);
            if let Some(f) = self.frames.last_mut() {
                f.scope = scope;
            }
            let target = self.resolve_text(aml, &text[4], at);
            if let (Some(f), Some(s)) = (self.frames.last_mut(), saved) {
                f.scope = s;
            }
            self.write_named(aml, target?, data, at, false)?;
        }
        Ok(Object::Table(index))
    }
}
