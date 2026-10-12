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

//! One _PSS or _CST state from a MkAcpiCpu record; `cpu_layout` says the
//! bytes. A C-state's type is 1, 2 or 3 or the record is refused.

use super::cpu_record::{CState, PState};
use super::cpu_take::Take;

pub fn pstate(t: &mut Take<'_>) -> Option<PState> {
    Some(PState {
        mhz: t.u32()?,
        milliwatts: t.u32()?,
        latency_us: t.u32()?,
        bus_master_latency_us: t.u32()?,
        control: t.u32()?,
        status: t.u32()?,
    })
}

pub fn cstate(t: &mut Take<'_>) -> Option<CState> {
    let reg = t.register()?;
    let kind = u8::try_from(t.u32()?).ok().filter(|k| (1..=3).contains(k))?;
    Some(CState { reg, kind, latency_us: t.u32()?, milliwatts: t.u32()? })
}
