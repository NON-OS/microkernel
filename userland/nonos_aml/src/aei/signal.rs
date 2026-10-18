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

//! One pin's event served. For a pin numbered up to 255 the controller's
//! _Exx (edge) or _Lxx (level) runs, xx the pin in two upper-case hex
//! digits; where that method is absent, or the pin is higher, _EVT runs
//! with the pin's number. A pin _AEI does not list runs nothing. Which
//! method, and the fallback, are recalled from 5.6.5: unverified.

use super::pins::AeiPin;
use crate::exec::Fault;
use crate::load::Aml;
use crate::namespace::{Id, Seg};
use crate::object::Object;
use crate::platform::{child, Eval};
use alloc::vec;
use alloc::vec::Vec;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ran {
    Edge,
    Level,
    Evt,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AeiError {
    /// The pin is not one the controller's _AEI lists.
    NotListed,
    /// Neither the pin's own method nor _EVT exists.
    NoMethod,
    Fault(Fault),
}

/// `_Exx` or `_Lxx` for a pin that has one.
pub fn method_for(pin: u16, edge: bool) -> Option<Seg> {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    let pin = u8::try_from(pin).ok()?;
    let kind = if edge { b'E' } else { b'L' };
    Some([b'_', kind, HEX[usize::from(pin >> 4)], HEX[usize::from(pin & 15)]])
}

pub fn signal(
    aml: &mut Aml,
    eval: &mut Eval<'_>,
    controller: Id,
    listed: &[AeiPin],
    pin: u16,
) -> Result<Ran, AeiError> {
    let p = listed.iter().find(|p| p.pin == pin).ok_or(AeiError::NotListed)?;
    let own = method_for(pin, p.edge).filter(|s| aml.ns.child(controller, *s).is_some());
    let (seg, args, ran): (Seg, Vec<Object>, Ran) = match own {
        Some(s) => (s, Vec::new(), if p.edge { Ran::Edge } else { Ran::Level }),
        None => (*b"_EVT", vec![Object::Integer(u64::from(pin))], Ran::Evt),
    };
    match child(aml, eval, controller, seg, args) {
        None => Err(AeiError::NoMethod),
        Some(r) => r.map(|_| ran).map_err(AeiError::Fault),
    }
}
