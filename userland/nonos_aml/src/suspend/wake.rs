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

//! Arming a device to wake the system from a sleeping state, and disarming
//! it after (ACPI 6.5, 7.3.13 and 7.3.1): its _PRW power resources on, then
//! _DSW(1, state, D3), or the older _PSW(1) where _DSW is absent, then its
//! GPE enabled; undone in the reverse order. Only devices the owner's
//! policy names are armed, and only for a state their _PRW reaches.

use super::hardware::SleepHardware;
use super::prw::read_prw;
use super::unwake::enable;
use crate::load::Aml;
use crate::namespace::Id;
use crate::platform::{child, Eval};
use alloc::vec::Vec;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Armed {
    pub dev: Id,
    pub gpe: Option<u16>,
    pub resources: Vec<Id>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Skip {
    NoPrw,
    /// Its _PRW wakes from no deeper than this state.
    TooShallow(u8),
}

pub fn arm(
    aml: &mut Aml,
    eval: &mut Eval<'_>,
    hw: &mut impl SleepHardware,
    dev: Id,
    state: u8,
) -> Result<Armed, Skip> {
    let prw = read_prw(aml, eval, dev).ok_or(Skip::NoPrw)?;
    if prw.deepest < state {
        return Err(Skip::TooShallow(prw.deepest));
    }
    for r in &prw.resources {
        let _ = child(aml, eval, *r, *b"_ON_", Vec::new());
    }
    enable(aml, eval, dev, true, state);
    if let Some(g) = prw.gpe {
        let _ = hw.arm_gpe(g, true);
    }
    Ok(Armed { dev, gpe: prw.gpe, resources: prw.resources })
}
