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

//! _BTP (ACPI 6.5, 10.2.2.10): the capacity at which the battery notifies
//! 0x80, set to its own warning level, so a battery crossing into low is
//! read when it happens rather than at the next minute's poll. _BTP is
//! optional; a battery without it is only polled.

use super::state::Battery;
use crate::boot::evaluate;
use crate::host::CapsuleHost;
use alloc::vec;
use nonos_aml::load::Aml;
use nonos_aml::object::Object;

pub fn set_trip(b: &Battery, aml: &mut Aml, host: &mut CapsuleHost) {
    let Some(warning) = b.info.as_ref().and_then(|i| i.warning).filter(|w| *w > 0) else { return };
    if let Some(btp) = aml.ns.child(b.id, *b"_BTP") {
        let _ = evaluate(aml, host, btp, vec![Object::Integer(u64::from(warning))]);
    }
}
