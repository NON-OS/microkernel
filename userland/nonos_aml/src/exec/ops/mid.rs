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

//! Mid (section 19.6.81): a piece of a string or a buffer.

use crate::exec::{Fault, FaultKind, Host, Machine};
use crate::load::Aml;
use crate::object::Object;

impl<H: Host> Machine<'_, H> {
    /// A start past the end gives an empty result, and a length past the
    /// end stops at it; an integer is read as a buffer.
    pub(crate) fn mid(&mut self, aml: &mut Aml, t: u16, node: u32) -> Result<Object, Fault> {
        let at = aml.node(t, node).at;
        let k = aml.kids(t, node);
        let src = self.value(aml, t, k[0])?;
        let (start, len) = (self.int(aml, t, k[1])?, self.int(aml, t, k[2])?);
        let w = self.width(aml, t);
        let bytes = src.as_bytes(w).ok_or_else(|| self.fault(FaultKind::Type, at))?;
        let from = usize::try_from(start).unwrap_or(usize::MAX).min(bytes.len());
        let to = from.saturating_add(usize::try_from(len).unwrap_or(usize::MAX)).min(bytes.len());
        let piece = &bytes[from..to];
        self.charge(piece.len() as u64, at)?;
        let out = match src {
            Object::String(_) => Object::string(piece),
            _ => Object::buffer(piece.to_vec()),
        };
        self.result(aml, t, k[3], out, at)
    }
}
