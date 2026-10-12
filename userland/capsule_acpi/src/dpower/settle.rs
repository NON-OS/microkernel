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

//! The end of boot for what `hidden` revealed: once, a minute after start,
//! every revealed function no driver has claimed goes back to D3cold and
//! gives its power resources back.

use super::hidden::Bdf;
use crate::boot::{evaluator, path, Acpi};
use crate::sys::acpi_holds::claimed_by_anyone;
use crate::sys::clock::now_ms;
use crate::sys::log::Line;
use alloc::vec::Vec;
use nonos_aml::device_power::settle;
use nonos_aml::namespace::Id;

#[derive(Default)]
pub struct Hidden {
    pub answered: Vec<(Id, Bdf)>,
    pub settle_at: u64,
    pub settled: bool,
}

pub fn tick(acpi: &mut Acpi) {
    let h = &acpi.hidden;
    if h.settled || h.answered.is_empty() || now_ms() < h.settle_at {
        return;
    }
    let answered = acpi.hidden.answered.clone();
    let ids: Vec<Id> = answered.iter().map(|(d, _)| *d).collect();
    let bdf = |d: Id| answered.iter().find(|(x, _)| *x == d).map(|(_, f)| *f);
    let mut claimed = |d: Id| bdf(d).is_some_and(|f| claimed_by_anyone(f) == Ok(true));
    let Acpi { aml, host, dpower, cooling, .. } = acpi;
    let down = settle(dpower, &mut cooling.pool, aml, &mut evaluator(host), &ids, &mut claimed);
    for d in &down {
        Line::new(b"hidden ")
            .str(path(aml, *d).as_bytes())
            .str(b": unclaimed, back to D3cold")
            .end();
    }
    acpi.hidden.settled = true;
}
