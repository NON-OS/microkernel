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

//! _PMC, the meter's capabilities package: supported capabilities (bit 0
//! measurement, bit 1 trip points, bit 2 a hardware enforced cap), the
//! unit (0, milliwatts), the measurement type (0, input power), accuracy
//! (thousandths of a percent), sampling time and the averaging interval's
//! bounds (milliseconds), hysteresis (milliwatts), whether the cap can be
//! set and its bounds (milliwatts), then model, serial and OEM strings.
//! 0xFFFFFFFF reads as unknown. The element order is recalled from ACPI
//! 6.5, 10.4.1, not read: unverified.

use crate::object::Object;
use crate::power::fields::{elements, known, text};
use alloc::vec::Vec;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Pmc {
    pub capabilities: u32,
    pub unit: Option<u32>,
    pub kind: Option<u32>,
    pub accuracy: Option<u32>,
    pub sampling_ms: Option<u32>,
    pub min_interval_ms: Option<u32>,
    pub max_interval_ms: Option<u32>,
    pub hysteresis_mw: Option<u32>,
    pub cap_configurable: bool,
    pub min_cap_mw: Option<u32>,
    pub max_cap_mw: Option<u32>,
    pub model: Vec<u8>,
}

pub fn parse_pmc(o: &Object) -> Option<Pmc> {
    let e = elements(o)?;
    (e.len() >= 14).then(|| Pmc {
        capabilities: known(&e, 0).unwrap_or(0),
        unit: known(&e, 1),
        kind: known(&e, 2),
        accuracy: known(&e, 3),
        sampling_ms: known(&e, 4),
        min_interval_ms: known(&e, 5),
        max_interval_ms: known(&e, 6),
        hysteresis_mw: known(&e, 7),
        cap_configurable: known(&e, 8) == Some(1),
        min_cap_mw: known(&e, 9),
        max_cap_mw: known(&e, 10),
        model: text(&e, 11),
    })
}
