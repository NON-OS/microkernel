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
//! its idle states (_CST or _LPI) changed; 0x82, _TPC changed (recalled,
//! unverified); 0x83 and 0x85, CPPC's
//! guaranteed or highest level changed. On a processor container, 0x81
//! says the container's _LPI changed, which every processor under it
//! carries. Each reads the processors' tables again and, when they moved,
//! hands them over and logs the line anew.

use super::report::line;
use super::send::send;
use super::state::Cpus;
use crate::boot::evaluator;
use crate::host::CapsuleHost;
use crate::sys::log::Line;
use alloc::vec::Vec;
use nonos_aml::cpu::cpu_tables;
use nonos_aml::load::Aml;
use nonos_aml::namespace::Id;

const PPC_CHANGED: u8 = 0x80;
const IDLE_CHANGED: u8 = 0x81;
const TPC_CHANGED: u8 = 0x82;
const GUARANTEED_CHANGED: u8 = 0x83;
const HIGHEST_CHANGED: u8 = 0x85;

fn reread(cpus: &mut Cpus, aml: &mut Aml, host: &mut CapsuleHost, i: usize) {
    let tables = cpu_tables(aml, &mut evaluator(host), cpus.list[i].id);
    if tables != cpus.list[i].tables {
        cpus.list[i].tables = tables;
        send(&cpus.list[i]);
        line(&[cpus.list[i].uid], &cpus.list[i].tables);
    }
}

pub fn notified(cpus: &mut Cpus, aml: &mut Aml, host: &mut CapsuleHost, id: Id, value: u8) -> bool {
    let under = |c: &super::state::Cpu| {
        c.tables.lpi.iter().any(|l| l.container.is_some_and(|(k, _)| k == id))
    };
    let Some(i) = cpus.list.iter().position(|c| c.id == id) else {
        let held: Vec<usize> = (0..cpus.list.len()).filter(|i| under(&cpus.list[*i])).collect();
        if held.is_empty() {
            return false;
        }
        if value == IDLE_CHANGED {
            held.into_iter().for_each(|i| reread(cpus, aml, host, i));
        }
        return true;
    };
    let known = [PPC_CHANGED, IDLE_CHANGED, TPC_CHANGED, GUARANTEED_CHANGED, HIGHEST_CHANGED];
    if !known.contains(&value) {
        Line::new(b"cpu ").dec(cpus.list[i].uid).str(b" notify ").hex(u64::from(value)).end();
        return true;
    }
    reread(cpus, aml, host, i);
    true
}
