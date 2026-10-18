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

//! DBG2's devices: register counts and string offsets that lie, and device
//! counts past what the table holds.

use super::gen::{damage, sdt, Rng};
use super::sweep::{headed, sweep};
use crate::arm_acpi::dbg2::{dbg2, MAX_DEVICES, MAX_REGISTERS};

/// A device laid out honestly; when `hostile`, sometimes a count or offset lies.
fn device(r: &mut Rng, n: usize, path: usize, hostile: bool) -> Vec<u8> {
    let (regs_at, sizes_at, ns_at) = (22u8, 22 + 12 * n, 22 + 16 * n);
    let len = (ns_at + path) as u16;
    let mut d = vec![0, len as u8, (len >> 8) as u8, n as u8, path as u8, 0, ns_at as u8, 0];
    d.extend([0, 0, 0, 0, 0x00, 0x80, 0, 0, 0, 0, regs_at, 0, sizes_at as u8, 0]);
    d.extend(r.bytes(16 * n + path));
    if hostile && r.one_in(5) {
        let at = r.pick(&[3, 4, 6, 18, 20]);
        d[at] = r.next() as u8;
    }
    d
}

fn dbg2_table(r: &mut Rng) -> Vec<u8> {
    let (k, regs) = (r.below(5), r.below(MAX_REGISTERS + 3));
    let devices: Vec<u8> = (0..k)
        .flat_map(|_| {
            let path = r.below(12);
            device(r, regs, path, true)
        })
        .collect();
    let any = r.next() as u32;
    let count = r.pick(&[0, k as u32, k as u32 + 1, 17, any]);
    let body = [&44u32.to_le_bytes()[..], &count.to_le_bytes(), &devices].concat();
    damage(sdt(b"DBG2", 0, &body), r)
}

#[test]
fn dbg2_devices_load_or_are_refused() {
    sweep(dbg2_table, headed(dbg2), |d, t| {
        d.len() <= MAX_DEVICES
            && d.iter().all(|d| d.registers.len() <= MAX_REGISTERS && d.namespace.len() < t.len())
    });
}

#[test]
fn dbg2_stops_at_its_caps() {
    let r = &mut Rng::new(98);
    let devices: Vec<u8> =
        (0..MAX_DEVICES + 4).flat_map(|_| device(r, MAX_REGISTERS + 2, 4, false)).collect();
    let body = [&44u32.to_le_bytes()[..], &20u32.to_le_bytes(), &devices].concat();
    let d = dbg2(&sdt(b"DBG2", 0, &body)).unwrap();
    assert_eq!(d.len(), MAX_DEVICES);
    assert!(d.iter().all(|d| d.registers.len() == MAX_REGISTERS));
}
