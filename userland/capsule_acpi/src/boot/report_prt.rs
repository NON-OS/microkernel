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

//! Each device's _PRT, one line per route.

use super::eval::evaluator;
use super::report::path;
use crate::host::CapsuleHost;
use crate::sys::log::Line;
use alloc::vec::Vec;
use nonos_aml::load::Aml;
use nonos_aml::namespace::Id;
use nonos_aml::platform::{routes, DeviceInfo, Route};

pub(super) fn report_routes(
    aml: &mut Aml,
    host: &mut CapsuleHost,
    devices: &[DeviceInfo],
) -> Vec<(Id, Vec<Route>)> {
    let mut all = Vec::new();
    for d in devices {
        match routes(aml, &mut evaluator(host), d.id) {
            None => {}
            Some(Ok(r)) => {
                for x in &r {
                    let l = Line::new(b"prt ")
                        .str(path(aml, d.id).as_bytes())
                        .str(b" slot=")
                        .dec(x.slot as u64);
                    let l = l.str(b" pin=").dec(x.pin as u64).str(b" gsi=");
                    match x.gsi {
                        Some(g) => l.dec(g as u64).end(),
                        None => l.str(b"none").end(),
                    }
                }
                all.push((d.id, r));
            }
            Some(Err(f)) => Line::new(b"_PRT faulted under ")
                .str(path(aml, d.id).as_bytes())
                .str(b" at ")
                .hex(f.at as u64)
                .end(),
        }
    }
    all
}
