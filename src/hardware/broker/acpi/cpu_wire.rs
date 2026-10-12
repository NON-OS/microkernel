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

//! Decoding a MkAcpiCpu record. The capsule is trusted to have read the
//! AML, not to have written the record well: counts, flags, register kinds
//! and spaces are all checked, and the record must end where it says.

use super::cpu_layout::*;
use super::cpu_record::{CState, CpuTables, PState, Reg};
use super::cpu_take::Take;
use alloc::vec::Vec;

fn pstate(t: &mut Take<'_>) -> Option<PState> {
    Some(PState {
        mhz: t.u32()?,
        milliwatts: t.u32()?,
        latency_us: t.u32()?,
        bus_master_latency_us: t.u32()?,
        control: t.u32()?,
        status: t.u32()?,
    })
}

fn cstate(t: &mut Take<'_>) -> Option<CState> {
    let reg = t.register()?;
    let kind = u8::try_from(t.u32()?).ok().filter(|k| (1..=3).contains(k))?;
    Some(CState { reg, kind, latency_us: t.u32()?, milliwatts: t.u32()? })
}

fn cpc(t: &mut Take<'_>) -> Option<(u8, [Reg; CPC_FIELDS])> {
    let revision = t.u8()?;
    t.zeros::<7>()?;
    let mut fields = [Reg::default(); CPC_FIELDS];
    for f in fields.iter_mut() {
        *f = t.reg()?;
    }
    Some((revision, fields))
}

pub fn decode(b: &[u8]) -> Option<CpuTables> {
    let mut t = Take { b, at: 0 };
    let [version, np, nc, flags] = t.bytes::<4>()?;
    t.zeros::<4>()?;
    let (uid, ppc) = (t.u64()?, t.u64()?);
    let (known, unset_ppc) = (HAS_PCT | HAS_PPC | HAS_CPC, flags & HAS_PPC == 0 && ppc != 0);
    let counts = np as usize <= MAX_PSTATES && nc as usize <= MAX_CSTATES;
    if version != VERSION || flags & !known != 0 || unset_ppc || !counts {
        return None;
    }
    let pct = match flags & HAS_PCT {
        0 => None,
        _ => Some([t.register()?, t.register()?]),
    };
    let pstates = (0..np).map(|_| pstate(&mut t)).collect::<Option<Vec<PState>>>()?;
    let cstates = (0..nc).map(|_| cstate(&mut t)).collect::<Option<Vec<CState>>>()?;
    let cpc = match flags & HAS_CPC {
        0 => None,
        _ => Some(cpc(&mut t)?),
    };
    let ppc = (flags & HAS_PPC != 0).then_some(ppc);
    (t.at == b.len()).then_some(CpuTables { uid, ppc, pct, pstates, cstates, cpc })
}
