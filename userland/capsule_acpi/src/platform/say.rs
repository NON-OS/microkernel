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

//! One line per listed device the walk saw: published with its broker id,
//! refused with the kernel's reason, or passed over with the capsule's.

use super::record::MAX_WINDOWS;
use crate::sys::acpi::Errno;
use crate::sys::log::Line;
use nonos_aml::platform_device::{Class, PlatformDevice, Skip};

const EACCES: Errno = -13;
const EBUSY: Errno = -16;
const EINVAL: Errno = -22;

fn class(c: Class) -> &'static [u8] {
    match c {
        Class::I2c => b"i2c",
        Class::Sd => b"sd",
        Class::Gpio => b"gpio",
    }
}

pub fn found(path: &[u8], d: &PlatformDevice, source: &[u8], got: Result<u64, Errno>) {
    let l = Line::new(b"platform ").str(path).str(b" ").str(&d.matched).str(b" ");
    let l = l.str(class(d.class)).str(b" windows=").dec(d.windows.len() as u64);
    let l = match d.irq {
        Some(i) => l.str(b" gsi=").dec(u64::from(i.gsiv)),
        None => l.str(b" no interrupt"),
    };
    let l = match got {
        Ok(id) => l.str(b" device=").hex(id),
        Err(EBUSY) => l.str(b": refused, a window overlaps a device already in the table"),
        Err(EACCES) => l.str(b": refused, a window is outside what the region rule grants"),
        Err(EINVAL) => l.str(b": refused, the record did not decode"),
        Err(e) => l.str(b": refused, errno ").dec(e.unsigned_abs()),
    };
    // The id list says where each id is documented; the line carries it.
    l.str(b"; id: ").str(source).end();
}

pub fn unsent(path: &[u8], d: &PlatformDevice) {
    let l = Line::new(b"platform ").str(path).str(b" ").str(&d.matched);
    let l = l.str(b": not sent, the kernel would refuse it: more than ").dec(MAX_WINDOWS as u64);
    l.str(b" windows, or a path past 127 bytes").end();
}

pub fn skipped(path: &[u8], why: Skip) {
    let why: &[u8] = match why {
        Skip::NoTemplate => b"_CRS failed or gave no buffer",
        Skip::NoWindow => b"_CRS names no memory window",
    };
    Line::new(b"platform ").str(path).str(b" passed over: ").str(why).end();
}
