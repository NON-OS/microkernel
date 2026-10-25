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

//! A SERIAL reply read field by field; a reply shorter than its own fields
//! say is no record.

use super::at::At;
use super::wire::{HidFacts, Serial, HID_NEEDED};

/// The record in `reply`, or the status it was refused with.
pub fn parse(reply: &[u8]) -> Option<Result<Serial, i32>> {
    let mut a = At(reply, 0);
    let status = a.u32()? as i32;
    if status != 0 {
        return Some(Err(status));
    }
    let (count, kind, flags, chip_select, hz) = (a.u32()?, a.u8()?, a.u8()?, a.u16()?, a.u32()?);
    let (_, pin, _, _, gpio_uid) = (a.u16()?, a.u32()?, a.u64()?, a.u64()?, a.u64()?);
    for _ in 0..4 {
        let n = a.u8()? as usize;
        a.take(n)?;
    }
    let key = a.u64()?;
    let (data_bits, phase, polarity, mode) = (a.u8()?, a.u8()?, a.u8()?, a.u8()?);
    let present = a.u8()?;
    let facts = HidFacts {
        input_header: a.u32()?,
        input_body: a.u32()?,
        output: a.u32()?,
        read_opcode: a.u8()?,
        write_opcode: a.u8()?,
        flags: a.u16()?,
    };
    let hid = (present & HID_NEEDED == HID_NEEDED).then_some(facts);
    Some(Ok(Serial {
        count,
        kind,
        flags,
        chip_select,
        hz,
        pin,
        gpio_uid,
        key,
        data_bits,
        phase,
        polarity,
        mode,
        hid,
    }))
}
