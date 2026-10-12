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

//! A Create operator's operands made into a buffer field.

use crate::exec::{BufferField, Fault, FaultKind, Host, Machine};
use crate::load::Aml;
use crate::object::Object;
use crate::op::Op;

impl<H: Host> Machine<'_, H> {
    pub(crate) fn make_buffer_field(
        &mut self,
        aml: &mut Aml,
        t: u16,
        node: u32,
        at: u32,
    ) -> Result<BufferField, Fault> {
        let k = aml.kids(t, node);
        let Object::Buffer(buf) = self.value(aml, t, k[0])? else {
            return Err(self.fault(FaultKind::Type, at));
        };
        let index = self.int(aml, t, k[1])?;
        let (bit, bits) = match aml.node(t, node).op {
            Op::CREATE_FIELD => (index, self.int(aml, t, k[2])?),
            Op::CREATE_BIT_FIELD => (index, 1),
            Op::CREATE_BYTE_FIELD => (index.saturating_mul(8), 8),
            Op::CREATE_WORD_FIELD => (index.saturating_mul(8), 16),
            Op::CREATE_DWORD_FIELD => (index.saturating_mul(8), 32),
            _ => (index.saturating_mul(8), 64),
        };
        let end = bit.checked_add(bits).ok_or_else(|| self.fault(FaultKind::Index, at))?;
        if bits == 0 || bits > u32::MAX as u64 || end > buf.borrow().len() as u64 * 8 {
            return Err(self.fault(FaultKind::Index, at));
        }
        Ok(BufferField { buf, bit, bits: bits as u32 })
    }
}
