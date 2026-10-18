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

//! POWER_METER (19): every present meter, read-only. The body: a count u8,
//! then for each (at most four) a presence byte (bit 0 _PMC, 1 the power,
//! 2 the averaging interval, 3 the cap), _PMC's capabilities, unit,
//! accuracy, sampling time, the interval's bounds and hysteresis u32, a
//! settable-cap byte, the cap's bounds u32, then the power (mW), interval
//! (ms) and cap (mW) u32, and the model (length u8, bytes). A value that
//! is unknown is 0xFFFFFFFF. No meter is a count of 0.

use crate::boot::{evaluator, Acpi};
use alloc::vec::Vec;
use nonos_aml::power_meter::{meters, read_meter, Pmc};

const MAX_METERS: usize = 4;
const UNKNOWN: u32 = 0xFFFF_FFFF;

fn put(b: &mut Vec<u8>, v: Option<u32>) {
    b.extend_from_slice(&v.unwrap_or(UNKNOWN).to_le_bytes());
}

fn pmc(b: &mut Vec<u8>, p: &Pmc) {
    put(b, Some(p.capabilities));
    for v in [p.unit, p.accuracy, p.sampling_ms, p.min_interval_ms, p.max_interval_ms] {
        put(b, v);
    }
    put(b, p.hysteresis_mw);
    b.push(u8::from(p.cap_configurable));
    put(b, p.min_cap_mw);
    put(b, p.max_cap_mw);
}

pub fn power_meter(acpi: &mut Acpi) -> (i32, Vec<u8>) {
    let found = meters(&acpi.devices);
    let Acpi { aml, host, .. } = acpi;
    let mut b = alloc::vec![found.len().min(MAX_METERS) as u8];
    for dev in found.into_iter().take(MAX_METERS) {
        let m = read_meter(aml, &mut evaluator(host), dev);
        let small = |v: Option<u64>| v.and_then(|v| u32::try_from(v).ok());
        let has =
            [m.pmc.is_some(), m.power_mw.is_some(), m.interval_ms.is_some(), m.cap_mw.is_some()];
        b.push(has.iter().enumerate().fold(0, |f, (i, h)| f | u8::from(*h) << i));
        pmc(&mut b, &m.pmc.clone().unwrap_or_default());
        for v in [m.power_mw, m.interval_ms, m.cap_mw] {
            put(&mut b, small(v));
        }
        let model = m.pmc.map(|p| p.model).unwrap_or_default();
        let model = &model[..model.len().min(32)];
        b.push(model.len() as u8);
        b.extend_from_slice(model);
    }
    (0, b)
}
