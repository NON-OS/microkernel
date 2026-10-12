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

//! Device initialization (ACPI 6.5, 6.5.1): \_SB._INI first, then every
//! device's _INI, parents before children, depth first in definition order.
//! A device present runs its _INI and is looked into; one functioning but
//! not present is looked into without its _INI; any other is skipped with
//! all below it. A failing _STA counts as neither, and is reported.

use super::eval::Eval;
use super::ini_report::{run_ini, IniReport};
use super::sta::{sta, FUNCTIONING, PRESENT};
use super::walk::walk;
use crate::load::Aml;
use crate::namespace::{Kind, ROOT};

fn is_device(k: &Kind) -> bool {
    matches!(k, Kind::Device | Kind::Processor | Kind::ThermalZone)
}

pub fn initialize(aml: &mut Aml, eval: &mut Eval<'_>) -> IniReport {
    let mut r = IniReport::default();
    if let Some(sb) = aml.ns.child(ROOT, *b"_SB_") {
        run_ini(aml, eval, sb, &mut r);
    }
    walk(aml, &mut |aml, c| {
        let Some(kind) = aml.ns.get(c).map(|e| e.kind) else { return false };
        if !is_device(&kind) {
            return kind.is_container();
        }
        let s = sta(aml, eval, c).unwrap_or_else(|f| {
            r.sta_failed.push((c, f));
            0
        });
        if s & PRESENT != 0 {
            run_ini(aml, eval, c, &mut r);
        }
        let below = s & (PRESENT | FUNCTIONING) != 0;
        if !below {
            r.absent.push(c);
        }
        below
    });
    r
}
