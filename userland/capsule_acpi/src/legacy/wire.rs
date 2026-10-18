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

//! LEGACY (11, index u32): the legacy devices the namespace declares, for
//! the PS/2 driver, the UART host and the clock. Every reply, even ENOENT
//! past the last device, starts with the count u32, the 8042 verdict u8
//! (1 there, 0 not: then leave 0x60 and 0x64 alone) and the FADT's boot
//! flags u16 (ACPI 6.5, 5.2.9.3; 0xFFFF when the FADT has none). Then, for
//! the device asked: kind u8 (1 keyboard, 2 mouse, 3 UART, 4 RTC, 5 HPET,
//! 6 PIT), _UID u64 (all ones if none); up to 4 I/O ranges (count u8, then
//! base u32 and length u16 each); up to 2 memory ranges (count u8, base
//! u64 and length u32 each); up to 4 interrupts (count u8, GSI u32 and
//! flags u8 each: 1 edge, 2 active low, 4 shared, 8 wake); the path's
//! length u8 and bytes.

use super::wire_head::head;
use crate::boot::{path, Acpi};
use crate::server::protocol::ENOENT;
use alloc::vec::Vec;

pub fn legacy(acpi: &Acpi, index: Option<usize>) -> (i32, Vec<u8>) {
    let mut v = head(&acpi.legacy);
    let Some(d) = index.and_then(|i| acpi.legacy.list.get(i)) else { return (ENOENT, v) };
    v.push(d.kind as u8 + 1);
    v.extend_from_slice(&d.uid.unwrap_or(u64::MAX).to_le_bytes());
    let io: Vec<_> = d.io.iter().take(4).collect();
    v.push(io.len() as u8);
    io.iter().for_each(|r| {
        v.extend_from_slice(&(r.base as u32).to_le_bytes());
        v.extend_from_slice(&(r.len as u16).to_le_bytes());
    });
    let mem: Vec<_> = d.memory.iter().take(2).collect();
    v.push(mem.len() as u8);
    mem.iter().for_each(|r| {
        v.extend_from_slice(&r.base.to_le_bytes());
        v.extend_from_slice(&(r.len as u32).to_le_bytes());
    });
    let irqs: Vec<_> = d.irqs.iter().take(4).collect();
    v.push(irqs.len() as u8);
    for i in irqs {
        v.extend_from_slice(&i.gsiv.to_le_bytes());
        v.push(
            u8::from(i.edge)
                | u8::from(i.active_low) << 1
                | u8::from(i.shared) << 2
                | u8::from(i.wake) << 3,
        );
    }
    let p = path(&acpi.aml, d.id);
    let p = &p.as_bytes()[..p.len().min(200)];
    v.push(p.len() as u8);
    v.extend_from_slice(p);
    (0, v)
}
