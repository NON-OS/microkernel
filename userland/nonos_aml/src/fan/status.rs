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

//! A fan's information, _FIF (ACPI 6.5, 11.3.1), and its present state,
//! _FST (11.3.4). Integers follow the battery packages' rule: 0xFFFFFFFF
//! means unknown.

use crate::object::Object;
use crate::power::fields::{elements, known};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FanInfo {
    /// _FSL takes a percentage, 0 to 100, rather than an _FPS control value.
    pub fine_grain: bool,
    /// The step between useful percentages, under fine-grain control.
    pub step: u32,
    /// Whether the fan notifies 0x80 when it runs below the speed asked.
    pub low_speed_notify: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FanStatus {
    /// The control value _FSL last set.
    pub control: Option<u32>,
    pub rpm: Option<u32>,
}

pub fn parse_fif(o: &Object) -> Option<FanInfo> {
    let e = elements(o)?;
    (e.len() >= 4).then(|| FanInfo {
        fine_grain: known(&e, 1).is_some_and(|v| v != 0),
        step: known(&e, 2).unwrap_or(0),
        low_speed_notify: known(&e, 3).is_some_and(|v| v != 0),
    })
}

pub fn parse_fst(o: &Object) -> Option<FanStatus> {
    let e = elements(o)?;
    (e.len() >= 3).then(|| FanStatus { control: known(&e, 1), rpm: known(&e, 2) })
}
