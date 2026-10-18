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

//! A processor's _LPI levels at the end of its MkAcpiCpu record; the
//! kernel's `broker::acpi::lpi_wire` is the other half. A level with a
//! register the kernel would refuse is cut, and every level above it with
//! it, since the level below names their states.

use super::record_reg::{fits, put_absent, put_integer, register};
use alloc::vec::Vec;
use nonos_aml::cpu::{LpiEntry, LpiLevel, Register};

const OWN: u8 = 0;
const CONTAINER: u8 = 1;

fn level_fits(l: &LpiLevel) -> bool {
    l.lpi.states.iter().all(|s| {
        let entry = match s.entry {
            LpiEntry::Register(r) => fits(&r),
            LpiEntry::Integer(_) => true,
        };
        entry && [s.residency, s.usage].iter().flatten().all(fits)
    })
}

/// The levels the record can carry: the run from the processor's own up
/// to the first that does not fit.
pub fn usable(levels: &[LpiLevel]) -> &[LpiLevel] {
    &levels[..levels.iter().take_while(|l| level_fits(l)).count()]
}

fn counter(out: &mut Vec<u8>, r: &Option<Register>) {
    match r {
        Some(r) => register(out, r),
        None => put_absent(out),
    }
}

pub fn levels(out: &mut Vec<u8>, levels: &[LpiLevel]) {
    for l in levels {
        let (kind, uid) = l.container.map_or((OWN, 0), |(_, uid)| (CONTAINER, uid));
        out.extend_from_slice(&uid.to_le_bytes());
        out.extend_from_slice(&l.lpi.level.to_le_bytes());
        out.extend_from_slice(&[kind, l.lpi.states.len() as u8, 0, 0, 0, 0, 0, 0]);
        for s in &l.lpi.states {
            for w in [s.min_residency_us, s.wake_latency_us, s.flags, s.parent_state] {
                out.extend_from_slice(&w.to_le_bytes());
            }
            out.extend_from_slice(&s.context_lost.to_le_bytes());
            out.extend_from_slice(&s.counter_hz.to_le_bytes());
            match s.entry {
                LpiEntry::Register(r) => register(out, &r),
                LpiEntry::Integer(v) => put_integer(out, v),
            }
            counter(out, &s.residency);
            counter(out, &s.usage);
        }
    }
}
