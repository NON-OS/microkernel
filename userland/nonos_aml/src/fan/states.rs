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

//! A fan's performance states, _FPS (ACPI 6.5, 11.3.2): a revision, then
//! one package per state of Control, TripPoint, Speed, NoiseLevel and
//! Power. _FSL takes a state's Control value when the fan has no
//! fine-grain control.

use crate::object::Object;
use crate::power::fields::{elements, known};
use alloc::vec::Vec;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FanState {
    pub control: u32,
    pub rpm: Option<u32>,
    /// mW.
    pub power: Option<u32>,
}

pub fn parse_fps(o: &Object) -> Option<Vec<FanState>> {
    let e = elements(o)?;
    let state = |s: &Object| {
        let f = elements(s).filter(|f| f.len() >= 5)?;
        Some(FanState { control: known(&f, 0)?, rpm: known(&f, 2), power: known(&f, 4) })
    };
    (!e.is_empty()).then(|| e[1..].iter().filter_map(state).collect())
}

/// The _FSL argument for running flat out and for stopping: 100 and 0
/// under fine-grain control (11.3.3), else the highest and lowest _FPS
/// control values; None when neither is known.
pub fn levels(fine_grain: bool, states: &[FanState]) -> Option<(u64, u64)> {
    if fine_grain {
        return Some((100, 0));
    }
    let hi = states.iter().map(|s| s.control).max()?;
    let lo = states.iter().map(|s| s.control).min()?;
    Some((u64::from(hi), u64::from(lo)))
}
