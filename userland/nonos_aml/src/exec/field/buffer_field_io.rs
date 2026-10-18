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

//! Reading and writing a buffer field's bits in place.

use crate::exec::{Fault, Host, Machine};
use crate::load::Aml;
use crate::namespace::Id;
use crate::object::Object;
use crate::region::{get_bits, set_bits};
use alloc::vec::Vec;

impl<H: Host> Machine<'_, H> {
    pub(crate) fn buffer_field_read(
        &mut self,
        aml: &mut Aml,
        id: Id,
        at: u32,
    ) -> Result<Object, Fault> {
        let f = self.buffer_field(aml, id, at)?;
        let mut out = alloc::vec![0u8; (f.bits as usize).div_ceil(8)];
        let src: Vec<u8> = f.buf.borrow().clone();
        for i in (0..f.bits).step_by(64) {
            let n = (f.bits - i).min(64);
            set_bits(&mut out, i as u64, n, get_bits(&src, f.bit + i as u64, n));
        }
        self.field_value(aml, out, f.bits, at)
    }

    pub(crate) fn buffer_field_write(
        &mut self,
        aml: &mut Aml,
        id: Id,
        v: Object,
        at: u32,
    ) -> Result<(), Fault> {
        let f = self.buffer_field(aml, id, at)?;
        let src = self.field_source(aml, v, f.bits, at)?;
        let mut buf = f.buf.borrow_mut();
        for i in (0..f.bits).step_by(64) {
            let n = (f.bits - i).min(64);
            set_bits(&mut buf, f.bit + i as u64, n, get_bits(&src, i as u64, n));
        }
        Ok(())
    }
}
