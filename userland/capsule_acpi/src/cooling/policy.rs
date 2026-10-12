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

//! _SCP (ACPI 6.5, 11.4.13): the cooling policy a zone should follow. 0
//! asks for active cooling first, fans before slowing the processors; 1
//! for passive first, quieter and slower. NONOS asks for active: it runs
//! the fans, and slowing the processors waits on their performance control.

use crate::boot::evaluate;
use crate::host::CapsuleHost;
use alloc::vec;
use nonos_aml::load::Aml;
use nonos_aml::namespace::Id;
use nonos_aml::object::Object;

const ACTIVE: u64 = 0;

pub fn prefer_active(aml: &mut Aml, host: &mut CapsuleHost, tz: Id) {
    if let Some(scp) = aml.ns.child(tz, *b"_SCP") {
        let _ = evaluate(aml, host, scp, vec![Object::Integer(ACTIVE)]);
    }
}
