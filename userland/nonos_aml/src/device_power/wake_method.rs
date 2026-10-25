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

//! A device's runtime-wake methods (ACPI 6.5, chapter 7; recalled,
//! unverified): `_S0W`, the deepest D-state from which it can wake the
//! system while it runs (0 to 4, D0 to D3cold), and `_DSW(enable, 0,
//! D-state)`, which tells firmware the device is to wake from that state
//! in S0, or the older `_PSW(enable)` where `_DSW` is absent.

use super::state::DState;
use crate::exec::Fault;
use crate::load::Aml;
use crate::namespace::Id;
use crate::object::Object;
use crate::platform::{child, integer, Eval};
use alloc::vec;
use alloc::vec::Vec;

/// When `_S0W` is absent, a device is woken from no deeper than D3hot:
/// this capsule's choice, the state every PCI function signals from.
pub const S0W_ABSENT: DState = DState::D3Hot;

/// The deepest state it wakes from; `S0W_ABSENT` with no `_S0W`, and
/// None when `_S0W` faults or answers a value past D3cold.
pub fn s0w(aml: &mut Aml, eval: &mut Eval<'_>, dev: Id) -> Option<DState> {
    match child(aml, eval, dev, *b"_S0W", Vec::new()) {
        None => Some(S0W_ABSENT),
        Some(Ok(o)) => DState::from_wire(u8::try_from(integer(&o)?).ok()?),
        Some(Err(_)) => None,
    }
}

/// `_DSW(on, 0, d)` with both D3s as 3, else `_PSW(on)`; Ok when the
/// device has neither, since then firmware needs telling nothing.
pub fn wake_enable(
    aml: &mut Aml,
    eval: &mut Eval<'_>,
    dev: Id,
    on: bool,
    d: DState,
) -> Result<(), Fault> {
    let d = u64::from((d as u8).min(3)) * u64::from(on);
    let dsw = vec![Object::Integer(u64::from(on)), Object::Integer(0), Object::Integer(d)];
    let r = child(aml, eval, dev, *b"_DSW", dsw)
        .or_else(|| child(aml, eval, dev, *b"_PSW", vec![Object::Integer(u64::from(on))]));
    r.map_or(Ok(()), |r| r.map(|_| ()))
}
