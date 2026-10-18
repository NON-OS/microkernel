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

//! HID over SPI (Microsoft's HID over SPI protocol specification): a device
//! whose _HID or _CID is PNP0C51, whose report addresses and opcodes its
//! _DSM gives. The UUID, the revision and the function numbers are recalled
//! and unverified: 1 the input report header address, 2 the input report
//! body address, 3 the output report address, 4 the read opcode, 5 the
//! write opcode, 6 the flags. Only those are asked, and only those function
//! 0 says are there; any other function is never evaluated.

use super::hid_spi_types::{HidSpi, HID_SPI_DSM_UUID};
use crate::load::Aml;
use crate::namespace::Id;
use crate::object::Object;
use crate::platform::{child, Eval};
use alloc::vec;

const REVISION: u64 = 1;
const QUERY: u64 = 0;
const DOCUMENTED: core::ops::RangeInclusive<u64> = 1..=6;

fn dsm(aml: &mut Aml, eval: &mut Eval<'_>, dev: Id, f: u64) -> Option<Object> {
    let uuid = Object::buffer(HID_SPI_DSM_UUID.to_vec());
    let args = vec![uuid, Object::Integer(REVISION), Object::Integer(f), Object::package(vec![])];
    child(aml, eval, dev, *b"_DSM", args)?.ok()
}

/// Function 0's buffer: bit n set when function n is there.
fn offered(aml: &mut Aml, eval: &mut Eval<'_>, dev: Id) -> u64 {
    let Some(Object::Buffer(b)) = dsm(aml, eval, dev, QUERY) else { return 0 };
    let b = b.borrow();
    b.iter().take(8).enumerate().fold(0, |m, (i, x)| m | u64::from(*x) << (8 * i))
}

/// None when the _DSM offers none of the documented functions.
pub fn hid_spi(aml: &mut Aml, eval: &mut Eval<'_>, dev: Id) -> Option<HidSpi> {
    let mask = offered(aml, eval, dev);
    let mut int = |f: u64| match DOCUMENTED.contains(&f) && mask & (1 << f) != 0 {
        true => match dsm(aml, eval, dev, f)? {
            Object::Integer(v) => Some(v),
            _ => None,
        },
        false => None,
    };
    let h = HidSpi {
        input_header: int(1).and_then(|v| u32::try_from(v).ok()),
        input_body: int(2).and_then(|v| u32::try_from(v).ok()),
        output: int(3).and_then(|v| u32::try_from(v).ok()),
        read_opcode: int(4).and_then(|v| u8::try_from(v).ok()),
        write_opcode: int(5).and_then(|v| u8::try_from(v).ok()),
        flags: int(6).and_then(|v| u16::try_from(v).ok()),
    };
    (h != HidSpi::default()).then_some(h)
}
