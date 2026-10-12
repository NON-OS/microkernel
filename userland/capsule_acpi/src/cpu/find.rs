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

//! Every processor's tables, read at start and handed to the kernel.

use super::report::report;
use super::send::send;
use super::state::{Cpu, Cpus};
use crate::boot::evaluator;
use crate::host::CapsuleHost;
use nonos_aml::cpu::{cpu_tables, processors};
use nonos_aml::load::Aml;

pub fn find(aml: &mut Aml, host: &mut CapsuleHost) -> Cpus {
    let mut cpus = Cpus::default();
    let found = processors(aml, &mut evaluator(host));
    for p in found {
        let tables = cpu_tables(aml, &mut evaluator(host), p.id);
        cpus.list.push(Cpu { id: p.id, uid: p.uid, tables });
    }
    cpus.list.iter().for_each(send);
    report(&cpus);
    cpus
}
