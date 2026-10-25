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

//! Running the fans the zones call for (ACPI 6.5, 11.1.4): a fan runs when
//! any level listing it is on, and also when a zone listing it could not
//! be read, now or ever, since a fan left off over an unread zone is the worse mistake.

use super::run_fan::run_fan;
use super::state::Cooling;
use crate::boot::path;
use crate::host::CapsuleHost;
use crate::sys::log::Line;
use alloc::vec;
use alloc::vec::Vec;
use nonos_aml::load::Aml;

fn wanted(c: &Cooling) -> Vec<bool> {
    let mut want = vec![false; c.fans.len()];
    for z in &c.zones {
        for (x, fans) in z.levels.iter().enumerate() {
            let on = z.w.verdict.active.get(x).copied().unwrap_or(false);
            if z.w.temp.is_none() || z.w.unread || on {
                fans.iter().for_each(|&f| want[f] = true);
            }
        }
    }
    want
}

pub fn apply(c: &mut Cooling, aml: &mut Aml, host: &mut CapsuleHost) {
    for (i, on) in wanted(c).into_iter().enumerate() {
        if c.fans[i].on == Some(on) {
            continue;
        }
        let l = Line::new(b"fan ").str(path(aml, c.fans[i].id).as_bytes());
        match run_fan(&c.fans[i], &mut c.pool, aml, host, on) {
            Ok(()) => {
                c.fans[i].on = Some(on);
                l.str(if on { b" on" } else { b" off" }).end();
            }
            Err(f) => l.str(b" failed to switch at ").hex(f.at as u64).end(),
        }
    }
}
