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

//! Writing a field unit: whole units are written as they are, partial ones
//! by the field's UpdateRule (section 19.6.48).

use super::index_bank::Ctx;
use super::read::{room, space_of};
use crate::exec::{Fault, Host, Machine};
use crate::load::Aml;
use crate::namespace::Id;
use crate::object::Object;
use crate::region::{access_width, get_bits, merge, plan, UpdateRule};

impl<H: Host> Machine<'_, H> {
    pub(crate) fn field_write(
        &mut self,
        aml: &mut Aml,
        id: Id,
        v: Object,
        at: u32,
    ) -> Result<(), Fault> {
        let ctx = self.field_ctx(aml, id, at)?;
        if self.special(aml, &ctx, Some(v.clone()), at)?.is_some() {
            return Ok(());
        }
        let src = self.field_source(aml, v, ctx.unit.bit_len, at)?;
        self.locked(aml, ctx.flags, at, |m, aml| m.write_units(aml, &ctx, &src, at))
    }

    fn write_units(&mut self, aml: &mut Aml, ctx: &Ctx, src: &[u8], at: u32) -> Result<(), Fault> {
        self.select_bank(aml, ctx, at)?;
        let u = ctx.unit;
        let rule = UpdateRule::from_flags(ctx.flags);
        let w = access_width(u.access[0], u.bit_offset, u.bit_len, space_of(ctx), room(ctx));
        for p in plan(u.bit_offset, u.bit_len, w) {
            let value = get_bits(src, p.at as u64, p.bits);
            let word = if p.bits == w * 8 {
                value
            } else {
                let old = match rule {
                    UpdateRule::Preserve => self.unit_io(aml, ctx, p.byte, w, None, at)?,
                    _ => 0,
                };
                merge(old, value, p.shift, p.bits, rule)
            };
            // WriteAsOnes sets every bit outside the field, so the word is
            // cut to the access width before it reaches the host.
            let word = if w >= 8 { word } else { word & ((1u64 << (w * 8)) - 1) };
            self.unit_io(aml, ctx, p.byte, w, Some(word), at)?;
        }
        Ok(())
    }
}
