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

//! Reading a field unit (section 19.6.48): one access per unit it touches,
//! the Global Lock held around them if its LockRule asks.

use super::index_bank::{Ctx, Target};
use crate::exec::{Fault, Host, Machine};
use crate::load::Aml;
use crate::namespace::Id;
use crate::object::Object;
use crate::region::{access_width, plan, set_bits, Space};
use alloc::vec;

/// FieldFlags bit 4, LockRule: Lock.
pub(crate) const LOCK: u8 = 0x10;

impl<H: Host> Machine<'_, H> {
    pub(crate) fn field_read(&mut self, aml: &mut Aml, id: Id, at: u32) -> Result<Object, Fault> {
        let ctx = self.field_ctx(aml, id, at)?;
        if let Some(v) = self.special(aml, &ctx, None, at)? {
            return Ok(v);
        }
        self.locked(aml, ctx.flags, at, |m, aml| m.read_units(aml, &ctx, at))
    }

    fn read_units(&mut self, aml: &mut Aml, ctx: &Ctx, at: u32) -> Result<Object, Fault> {
        self.select_bank(aml, ctx, at)?;
        let u = ctx.unit;
        let w = access_width(u.access[0], u.bit_offset, u.bit_len, space_of(ctx), room(ctx));
        self.charge(u.bit_len as u64 / 8, at)?;
        let mut bytes = vec![0u8; (u.bit_len as usize).div_ceil(8)];
        for p in plan(u.bit_offset, u.bit_len, w) {
            let raw = self.unit_io(aml, ctx, p.byte, w, None, at)?;
            set_bits(&mut bytes, p.at as u64, p.bits, raw >> p.shift);
        }
        self.field_value(aml, bytes, u.bit_len, at)
    }
}

/// The bytes an access may reach: the region's length; an IndexField's
/// data register bounds its own accesses.
pub(crate) fn room(ctx: &Ctx) -> u64 {
    match &ctx.target {
        Target::Region(_, r) => r.len,
        Target::Index { .. } => u64::MAX,
    }
}

/// The space an access width is chosen for; an IndexField's own rules come
/// from its AccessType alone.
pub(crate) fn space_of(ctx: &Ctx) -> Space {
    match &ctx.target {
        Target::Region(_, r) => r.space,
        Target::Index { .. } => Space(0xFF),
    }
}
