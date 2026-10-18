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

//! One PCI interrupt route, and one _PRT entry read from its package
//! (ACPI 6.5, 6.2.13): the address (its high word the device), the pin,
//! the source (Zero, or a link device) and the source index.

use super::eval::integer;
use crate::namespace::Id;
use crate::object::{Object, Ref};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Route {
    /// The device number on the bridge's bus (the high word of the address).
    pub slot: u16,
    /// 0 to 3, INTA to INTD.
    pub pin: u8,
    /// The link device, when the entry names one.
    pub link: Option<Id>,
    /// None when a link's current resources name no interrupt.
    pub gsi: Option<u32>,
}

/// Where an entry's interrupt comes from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Source {
    /// Source Zero: the index is the GSI itself; None when it does not fit.
    Gsi(Option<u32>),
    /// A link device, and the index into its current resources.
    Link(Id, u64),
}

/// One entry: (slot, pin, source). None for anything else: fewer than four
/// elements, one of the wrong type, an address past 32 bits or a pin past
/// INTD: each refused rather than cut to fit and routed to the wrong line.
pub fn entry(e: &[Object]) -> Option<(u16, u8, Source)> {
    let (address, pin, index) = (integer(e.first()?)?, integer(e.get(1)?)?, integer(e.get(3)?)?);
    let address = u32::try_from(address).ok()?;
    let pin = u8::try_from(pin).ok().filter(|p| *p <= 3)?;
    let source = match e.get(2)? {
        Object::Integer(0) => Source::Gsi(u32::try_from(index).ok()),
        Object::Ref(Ref::Named(link, _)) | Object::Named(link, _) => Source::Link(*link, index),
        _ => return None,
    };
    Some(((address >> 16) as u16, pin, source))
}
