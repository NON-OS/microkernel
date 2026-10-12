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

//! One processor's performance and idle tables, each evaluated where the
//! processor defines it. A method that faults or answers in the wrong shape
//! leaves its part empty; the rest stand.

use super::cpc::{parse_cpc, Cpc};
use super::cst::{parse_cst, CState};
use super::pct::{parse_pct, Pct};
use super::pss::{parse_pss, PState};
use crate::load::Aml;
use crate::namespace::{Id, Seg};
use crate::object::Object;
use crate::platform::{child, integer, Eval};
use alloc::vec::Vec;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CpuTables {
    pub pstates: Vec<PState>,
    pub pct: Option<Pct>,
    /// _PPC as firmware gave it; `ppc_index` holds it to the list.
    pub ppc: Option<u64>,
    pub cstates: Vec<CState>,
    pub cpc: Option<Cpc>,
}

fn get(aml: &mut Aml, eval: &mut Eval<'_>, cpu: Id, seg: Seg) -> Option<Object> {
    child(aml, eval, cpu, seg, Vec::new())?.ok()
}

/// _PPC alone, for the Notify 0x80 that says it changed (8.4.5.3).
pub fn ppc(aml: &mut Aml, eval: &mut Eval<'_>, cpu: Id) -> Option<u64> {
    get(aml, eval, cpu, *b"_PPC").as_ref().and_then(integer)
}

pub fn cpu_tables(aml: &mut Aml, eval: &mut Eval<'_>, cpu: Id) -> CpuTables {
    CpuTables {
        pstates: get(aml, eval, cpu, *b"_PSS").map(|o| parse_pss(&o)).unwrap_or_default(),
        pct: get(aml, eval, cpu, *b"_PCT").as_ref().and_then(parse_pct),
        ppc: ppc(aml, eval, cpu),
        cstates: get(aml, eval, cpu, *b"_CST").map(|o| parse_cst(&o)).unwrap_or_default(),
        cpc: get(aml, eval, cpu, *b"_CPC").as_ref().and_then(parse_cpc),
    }
}
