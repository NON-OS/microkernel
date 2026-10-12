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

//! A register or _CPC entry in MkAcpiCpu's 16 bytes: kind, space, width,
//! offset, access, three zero bytes, then the address or the integer. The
//! kernel refuses a register outside the spaces a processor control lives
//! in, or with an access size past QWord; `fits` says so first, so one bad
//! register costs its own table and not the processor's whole record.

use alloc::vec::Vec;
use nonos_aml::cpu::{CpcField, Register, FIXED_HARDWARE, PCC, SYSTEM_IO, SYSTEM_MEMORY};

const ABSENT: u8 = 0;
const REGISTER: u8 = 1;
const INTEGER: u8 = 2;

pub fn fits(r: &Register) -> bool {
    [SYSTEM_MEMORY, SYSTEM_IO, PCC, FIXED_HARDWARE].contains(&r.space) && r.access <= 4
}

fn put(out: &mut Vec<u8>, head: [u8; 5], value: u64) {
    out.extend_from_slice(&head);
    out.extend_from_slice(&[0; 3]);
    out.extend_from_slice(&value.to_le_bytes());
}

pub fn register(out: &mut Vec<u8>, r: &Register) {
    put(out, [REGISTER, r.space, r.width, r.offset, r.access], r.address);
}

pub fn field(out: &mut Vec<u8>, f: &CpcField) {
    match f {
        CpcField::Absent => put(out, [ABSENT, 0, 0, 0, 0], 0),
        CpcField::Integer(v) => put(out, [INTEGER, 0, 0, 0, 0], *v),
        CpcField::Register(r) => register(out, r),
    }
}

/// Whether every register entry of a _CPC fits.
pub fn cpc_fits(fields: &[CpcField]) -> bool {
    fields.iter().all(|f| match f {
        CpcField::Register(r) => fits(r),
        _ => true,
    })
}
