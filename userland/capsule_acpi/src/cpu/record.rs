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

//! A processor's tables as MkAcpiCpu takes them; the kernel's
//! `broker::acpi::cpu_layout` is the other half and says the layout.
//! Parts whose registers the kernel would refuse are left out here.

use super::record_reg::{cpc_fits, field, fits, register};
use alloc::vec::Vec;
use nonos_aml::cpu::{CpuTables, MAX_CSTATES, MAX_PSTATES};

const VERSION: u8 = 1;
const HAS_PCT: u8 = 1;
const HAS_PPC: u8 = 2;
const HAS_CPC: u8 = 4;

pub fn record(uid: u64, t: &CpuTables) -> Vec<u8> {
    let pct = t.pct.filter(|p| fits(&p.control) && fits(&p.status));
    let pstates = &t.pstates[..t.pstates.len().min(MAX_PSTATES)];
    let cstates: Vec<_> =
        t.cstates.iter().filter(|c| fits(&c.register)).take(MAX_CSTATES).collect();
    let cpc = t.cpc.filter(|c| cpc_fits(&c.fields));
    let flags = [(pct.is_some(), HAS_PCT), (t.ppc.is_some(), HAS_PPC), (cpc.is_some(), HAS_CPC)]
        .iter()
        .fold(0, |f, (on, bit)| if *on { f | bit } else { f });
    let mut out = alloc::vec![VERSION, pstates.len() as u8, cstates.len() as u8, flags, 0, 0, 0, 0];
    out.extend_from_slice(&uid.to_le_bytes());
    out.extend_from_slice(&t.ppc.unwrap_or(0).to_le_bytes());
    if let Some(p) = pct {
        register(&mut out, &p.control);
        register(&mut out, &p.status);
    }
    for p in pstates {
        for w in [p.mhz, p.milliwatts, p.latency_us, p.bus_master_latency_us, p.control, p.status] {
            out.extend_from_slice(&w.to_le_bytes());
        }
    }
    for c in cstates {
        register(&mut out, &c.register);
        for w in [u32::from(c.kind), c.latency_us, c.milliwatts] {
            out.extend_from_slice(&w.to_le_bytes());
        }
    }
    if let Some(c) = cpc {
        out.extend_from_slice(&[c.revision, 0, 0, 0, 0, 0, 0, 0]);
        c.fields.iter().for_each(|f| field(&mut out, f));
    }
    out
}
