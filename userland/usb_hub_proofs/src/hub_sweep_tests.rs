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

//! Hub descriptors built sound for 1 to 255 ports, spread over powers of
//! two, then told one lie: the head is taken exactly when the rule stated
//! here takes it, and only the bytes the hub sent are read.

use nonos_usb_hub::{parse_hub_descriptor, HubDescriptor};

use crate::sweep_rng::{rounds, Rng};

/// The rule from USB 2.0 11.23.2.1 and USB 3.2 10.15.2.1 (as recalled,
/// unverified): a head of 7 bytes of the hub's own type, ports named, and
/// a bLength holding DeviceRemovable (12 bytes on a SuperSpeed hub).
fn rule(d: &[u8], ss: bool) -> Option<HubDescriptor> {
    let full = if ss { 12 } else { 7 + (d.get(2).copied()? as usize + 1).div_ceil(8) };
    let kind = if ss { 0x2A } else { 0x29 };
    if d.len() < 7 || d[1] != kind || d[2] == 0 || (d[0] as usize) < full.max(7) {
        return None;
    }
    let think_time = if ss { 0 } else { (d[3] >> 5) & 3 };
    let power_on_ms = (u32::from(d[5]) * 2).max(100);
    Some(HubDescriptor { ports: d[2].min(15), ports_named: d[2], think_time, power_on_ms })
}

#[test]
fn a_hub_head_is_taken_exactly_as_the_rule_takes_it() {
    let (mut told, mut taken) = ([0u32; 6], 0u32);
    for seed in 1..rounds(100_000) {
        let mut r = Rng::new(seed);
        let ss = r.next() & 1 == 1;
        let k = r.range(0, 7);
        let ports = if ss { r.range(1, 15) } else { r.range(1 << k, (2 << k) - 1).min(255) };
        let full = if ss { 12 } else { 7 + 2 * (ports + 1).div_ceil(8) };
        let mut d: Vec<u8> = (0..full).map(|_| r.next() as u8).collect();
        (d[0], d[1], d[2]) = (full as u8, if ss { 0x2A } else { 0x29 }, ports as u8);
        assert!(rule(&d, ss).is_some(), "seed {seed}: built sound");
        let lie = r.range(0, 5);
        let mut n = 7;
        match lie {
            0 => d[0] = r.range(0, 6) as u8,
            1 => d[0] = r.range(7, (7 + (ports + 1).div_ceil(8)).max(12 * ss as usize) - 1) as u8,
            2 => d[1] = if r.next() & 1 == 1 { d[1] ^ 0x03 } else { r.next() as u8 & 0xF0 },
            3 => d[2] = 0,
            4 => n = r.range(0, 6),
            _ => {}
        }
        let got = parse_hub_descriptor(&d[..n], ss);
        assert_eq!(got, rule(&d[..n], ss), "seed {seed}: lie {lie}");
        assert!(lie == 5 || got.is_none(), "seed {seed}: lie {lie} was taken");
        assert!(got.is_none_or(|h| h.ports >= 1 && h.ports <= 15), "seed {seed}");
        (told[lie], taken) = (told[lie] + 1, taken + got.is_some() as u32);
    }
    for (n, lie) in told.iter().zip(0..) {
        assert!(cfg!(miri) || *n >= 10_000, "lie {lie} was told only {n} times");
    }
    assert!(cfg!(miri) || taken >= 10_000, "only {taken} sound heads taken");
}
