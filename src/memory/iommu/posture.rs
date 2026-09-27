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

//! The one line that says what the IOMMU is doing on this boot.

use super::query::capabilities;
use crate::sys::serial;

/*
 * Built from the capability query and nothing else, so the line and the
 * query cannot disagree. The boot matrix reads vendor= and enforcing=.
 */
pub fn report_posture() {
    let caps = capabilities();
    serial::print(b"[IOMMU] vendor=");
    serial::print(caps.vendor.name());
    serial::print(b" enforcing=");
    serial::print(flag(caps.enforcing));
    serial::print(b" aw=");
    serial::print_dec(u64::from(caps.address_width_bits));
    serial::print(b" ir=");
    serial::print(flag(caps.interrupt_remapping));
    serial::print(b" snoop=");
    serial::print(flag(caps.snoop_control));
    serial::print(b" pages=");
    serial::print_hex(caps.page_sizes);
    serial::print(b" domains=");
    serial::print_dec(u64::from(caps.domain_count));
    serial::println(b"");
}

const fn flag(on: bool) -> &'static [u8] {
    if on {
        b"1"
    } else {
        b"0"
    }
}
