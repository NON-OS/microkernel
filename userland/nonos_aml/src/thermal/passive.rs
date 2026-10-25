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

//! A zone's passive cooling (ACPI 6.5, 11.4): _TC1, _TC2 and _TSP for the
//! rule (`passive_rule`), and _PSL, the processors it slows.

use super::trip_read::value;
use crate::load::Aml;
use crate::namespace::{Id, Seg};
use crate::platform::{listed, Eval};
use alloc::vec::Vec;

pub use super::passive_rule::{constants, step, Constants, FULL, MAX_TC, MAX_TSP};

const TC1: Seg = *b"_TC1";
const TC2: Seg = *b"_TC2";
const TSP: Seg = *b"_TSP";

/// The zone's constants, None when any is absent or a lie.
pub fn passive_constants(aml: &mut Aml, eval: &mut Eval<'_>, tz: Id) -> Option<Constants> {
    let (tc1, tc2) = (value(aml, eval, tz, TC1), value(aml, eval, tz, TC2));
    constants(tc1, tc2, value(aml, eval, tz, TSP))
}

/// _PSL: the processors passive cooling slows for this zone.
pub fn passive_devices(aml: &mut Aml, eval: &mut Eval<'_>, tz: Id) -> Vec<Id> {
    listed(aml, eval, tz, *b"_PSL")
}
