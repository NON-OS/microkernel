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

//! The Low Power S0 Idle device (_HID PNP0D80) and which of the _DSM
//! function sets (`vendor.rs`) it offers for suspend-to-idle.

use super::vendor::Vendor;
use crate::load::Aml;
use crate::namespace::Id;
use crate::object::Object;
use crate::platform::{child, DeviceInfo, Eval, PRESENT};
use alloc::vec;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Lps0 {
    pub dev: Id,
    /// Function 0's answer for each UUID: bit n set when function n is
    /// there; 0 when the UUID is not supported at all.
    pub microsoft: u64,
    pub intel: u64,
}

pub fn dsm(aml: &mut Aml, eval: &mut Eval<'_>, dev: Id, v: Vendor, f: u64) -> Option<Object> {
    let args = vec![
        Object::buffer(v.uuid().to_vec()),
        Object::Integer(0),
        Object::Integer(f),
        Object::package(vec![]),
    ];
    child(aml, eval, dev, *b"_DSM", args)?.ok()
}

fn functions(aml: &mut Aml, eval: &mut Eval<'_>, dev: Id, v: Vendor) -> u64 {
    let mask = match dsm(aml, eval, dev, v, 0) {
        Some(Object::Buffer(b)) => {
            b.borrow().iter().take(8).rev().fold(0, |m, b| m << 8 | u64::from(*b))
        }
        Some(Object::Integer(n)) => n,
        _ => 0,
    };
    if mask & 1 != 0 {
        mask
    } else {
        0
    }
}

pub fn find_lps0(aml: &mut Aml, eval: &mut Eval<'_>, devices: &[DeviceInfo]) -> Option<Lps0> {
    let ids = |d: &&DeviceInfo| d.hid.iter().chain(&d.cids).any(|h| h.bytes() == b"PNP0D80");
    let dev = devices.iter().filter(|d| d.sta & PRESENT != 0).find(ids)?.id;
    let (microsoft, intel) =
        (functions(aml, eval, dev, Vendor::Microsoft), functions(aml, eval, dev, Vendor::Intel));
    Some(Lps0 { dev, microsoft, intel })
}
