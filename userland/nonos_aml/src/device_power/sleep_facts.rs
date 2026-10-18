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

//! What a device says about system sleep, read only, nothing armed (ACPI
//! 6.5, chapter 7; recalled, unverified): `_S3D` and `_S4D`, the
//! shallowest D-state it may be in during S3 or S4 (0 to 3); `_S0W`,
//! `_S3W` and `_S4W`, the deepest D-state it can wake the system from in
//! S0, S3 or S4 (0 to 4, D3cold last); and `_PRW`'s deepest sleeping
//! state it can wake from. Each None when absent, faulting or past range.

use super::state::DState;
use crate::load::Aml;
use crate::namespace::{Id, Seg};
use crate::platform::{child, integer, Eval};
use crate::suspend::read_prw;
use alloc::vec::Vec;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SleepFacts {
    pub s0w: Option<DState>,
    pub s3d: Option<DState>,
    pub s4d: Option<DState>,
    pub s3w: Option<DState>,
    pub s4w: Option<DState>,
    /// _PRW's sleeping state, 0 to 5.
    pub prw_deepest: Option<u8>,
}

fn value(aml: &mut Aml, eval: &mut Eval<'_>, dev: Id, seg: Seg) -> Option<u64> {
    child(aml, eval, dev, seg, Vec::new())?.ok().as_ref().and_then(integer)
}

/// `_SxD` names D0 to D3 only.
fn d(aml: &mut Aml, eval: &mut Eval<'_>, dev: Id, seg: Seg) -> Option<DState> {
    value(aml, eval, dev, seg).filter(|v| *v <= 3).and_then(|v| DState::from_wire(v as u8))
}

/// `_SxW` names D0 to D3cold.
fn w(aml: &mut Aml, eval: &mut Eval<'_>, dev: Id, seg: Seg) -> Option<DState> {
    value(aml, eval, dev, seg).and_then(|v| u8::try_from(v).ok()).and_then(DState::from_wire)
}

pub fn sleep_facts(aml: &mut Aml, eval: &mut Eval<'_>, dev: Id) -> SleepFacts {
    SleepFacts {
        s0w: w(aml, eval, dev, *b"_S0W"),
        s3d: d(aml, eval, dev, *b"_S3D"),
        s4d: d(aml, eval, dev, *b"_S4D"),
        s3w: w(aml, eval, dev, *b"_S3W"),
        s4w: w(aml, eval, dev, *b"_S4W"),
        prw_deepest: read_prw(aml, eval, dev).map(|p| p.deepest).filter(|s| *s <= 5),
    }
}

impl SleepFacts {
    /// Six bytes, one a fact, 0xFF where it is None: S0W, S3D, S4D, S3W,
    /// S4W as D-states (0 D0 to 4 D3cold), then _PRW's sleeping state.
    pub fn bytes(&self) -> [u8; 6] {
        let s = |x: Option<DState>| x.map_or(0xFF, |d| d as u8);
        let prw = self.prw_deepest.unwrap_or(0xFF);
        [s(self.s0w), s(self.s3d), s(self.s4d), s(self.s3w), s(self.s4w), prw]
    }
}
