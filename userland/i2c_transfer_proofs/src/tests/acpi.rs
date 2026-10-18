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

//! OP_ACPI_HID asked through the shipping handler, on a reply buffer the
//! size the server keeps, and the reply read back the way the HID driver's
//! query_acpi_hid reads it: the status, then the address, the descriptor
//! register, the GPIO pin and the HID_INFO bits, little-endian, in eight
//! bytes.

use nonos_libc::take_reply;

use super::kept::{reply_buffer, wiped};
use crate::driver::Driver;
use crate::protocol::{parse, Request, E_OK, OP_ACPI_HID};
use crate::server::handlers::acpi_hid;

/// One answer: the address, its descriptor register, the pin and the bits.
pub type Named = (u8, u16, u16, u8);

/// The reply's payload, status first, once it is seen to go to the caller
/// under the request's op and id and to leave the reply buffer wiped.
pub fn ask(d: &Driver) -> Vec<u8> {
    let mut out = reply_buffer();
    acpi_hid::handle(d, 42, &Request { op: OP_ACPI_HID, request_id: 9 }, &mut out);
    let (to, bytes) = take_reply().expect("the handler replied");
    assert!(wiped(&out), "the reply buffer kept the answer");
    let (echo, payload) = parse(&bytes).expect("the reply parses");
    assert_eq!((to, echo.op, echo.request_id), (42, OP_ACPI_HID, 9));
    payload.to_vec()
}

/// What one ask names, or None for an empty OK; any other reply fails.
pub fn named_once(d: &Driver) -> Option<Named> {
    let p = ask(d);
    assert_eq!(p[..4], E_OK.to_le_bytes(), "OP_ACPI_HID failed");
    let body = &p[4..];
    if body.is_empty() {
        return None;
    }
    assert_eq!(body.len(), 8, "the body is eight bytes");
    assert_eq!((body[1], body[7]), (0, 0), "a 7-bit address and a zero last byte");
    let half = |at: usize| u16::from_le_bytes([body[at], body[at + 1]]);
    Some((body[0], half(2), half(4), body[6]))
}
