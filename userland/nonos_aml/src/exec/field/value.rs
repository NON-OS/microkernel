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

//! A field's bits as a value, and a value as a field's bits (section
//! 19.3.5.7: a field unit or buffer field reads as an integer when it fits
//! the integer width, else as a buffer).

use crate::exec::{Fault, FaultKind, Host, Machine};
use crate::load::Aml;
use crate::object::Object;
use crate::region::get_bits;
use alloc::vec::Vec;

impl<H: Host> Machine<'_, H> {
    pub(crate) fn field_value(
        &mut self,
        aml: &Aml,
        bytes: Vec<u8>,
        bits: u32,
        at: u32,
    ) -> Result<Object, Fault> {
        let w = self.width(aml, 0);
        if bits as usize <= w.bytes() * 8 {
            return Ok(Object::Integer(get_bits(&bytes, 0, bits)));
        }
        self.charge(bytes.len() as u64, at)?;
        Ok(Object::buffer(bytes))
    }

    /// The value's bytes, little-endian, cut or zero-padded to the field.
    pub(crate) fn field_source(
        &mut self,
        aml: &mut Aml,
        v: Object,
        bits: u32,
        at: u32,
    ) -> Result<Vec<u8>, Fault> {
        let w = self.width(aml, 0);
        let v = self.deref(aml, v, at)?;
        let mut bytes = v.as_bytes(w).ok_or_else(|| self.fault(FaultKind::Type, at))?;
        bytes.resize((bits as usize).div_ceil(8), 0);
        Ok(bytes)
    }
}
