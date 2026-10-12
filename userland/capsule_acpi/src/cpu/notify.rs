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

//! A Notify on a processor (ACPI 6.5, 5.6.6): 0x80, _PPC changed; 0x81,
//! _CST changed; 0x83 and 0x85, CPPC's guaranteed or highest level
//! changed. Each reads the processor's tables again and, when they moved,
//! hands them over and logs the line anew.

use super::report::line;
use super::send::send;
use super::state::Cpus;
use crate::boot::evaluator;
use crate::host::CapsuleHost;
use crate::sys::log::Line;
use nonos_aml::cpu::cpu_tables;
use nonos_aml::load::Aml;
use nonos_aml::namespace::Id;

const PPC_CHANGED: u8 = 0x80;
const CST_CHANGED: u8 = 0x81;
const GUARANTEED_CHANGED: u8 = 0x83;
const HIGHEST_CHANGED: u8 = 0x85;

pub fn notified(cpus: &mut Cpus, aml: &mut Aml, host: &mut CapsuleHost, id: Id, value: u8) -> bool {
    let Some(i) = cpus.list.iter().position(|c| c.id == id) else { return false };
    if ![PPC_CHANGED, CST_CHANGED, GUARANTEED_CHANGED, HIGHEST_CHANGED].contains(&value) {
        Line::new(b"cpu ").dec(cpus.list[i].uid).str(b" notify ").hex(u64::from(value)).end();
        return true;
    }
    let tables = cpu_tables(aml, &mut evaluator(host), id);
    if tables != cpus.list[i].tables {
        cpus.list[i].tables = tables;
        send(&cpus.list[i]);
        line(&[cpus.list[i].uid], &cpus.list[i].tables);
    }
    true
}
