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

//! OP_CONTROLLER_INFO: the whole reply, field by field, in the layout
//! README.md gives it. It once copied an eight-byte value into a four-byte
//! slot and took the controller driver down on every call.

use nonos_i2cmodel::Config;
use nonos_libc::take_reply;

use super::fixture::bench;
use crate::protocol::{parse, Request, E_OK, IPC_PAYLOAD_MAX, OP_CONTROLLER_INFO};
use crate::server::handlers::controller::handle;

const CALLER: u32 = 42;

fn le(b: &[u8]) -> u64 {
    b.iter().rev().fold(0, |v, &x| (v << 8) | u64::from(x))
}

#[test]
fn the_controller_info_reply_carries_every_field_where_the_readme_puts_it() {
    let b = bench(Config::LPSS);
    let req = Request { op: OP_CONTROLLER_INFO, request_id: 9 };
    let mut out = [0u8; IPC_PAYLOAD_MAX];
    handle(&b.driver, CALLER, &req, &mut out);
    let (to, bytes) = take_reply().expect("the handler replied");
    assert_eq!(to, CALLER);
    let (echo, payload) = parse(&bytes).expect("reply header parses");
    assert_eq!((echo.op, echo.request_id), (OP_CONTROLLER_INFO, 9));
    assert_eq!(i32::from_le_bytes(payload[0..4].try_into().unwrap()), E_OK);
    let body = &payload[4..];
    assert_eq!(body.len(), 52, "device, PCI id, clock, epoch, MMIO grant, family");
    let d = &b.driver;
    assert_eq!(le(&body[0..8]), d.device_id);
    assert_eq!(le(&body[8..10]), u64::from(d.pci_device));
    assert_eq!(le(&body[10..14]), u64::from(d.setup.clock_hz));
    assert_eq!(le(&body[14..22]), d.claim_epoch);
    assert_eq!(le(&body[22..30]), d.mmio_grant);
    let name = d.family.as_bytes();
    assert_eq!(&body[30..30 + name.len()], name);
    assert!(body[30 + name.len()..].iter().all(|&c| c == 0), "the name is zero padded");
}

#[test]
fn a_family_name_longer_than_its_field_is_cut_at_the_field() {
    let mut b = bench(Config::LPSS);
    b.driver.family = "an LPSS family name far longer than twenty-two bytes";
    let mut out = [0u8; IPC_PAYLOAD_MAX];
    handle(&b.driver, CALLER, &Request { op: OP_CONTROLLER_INFO, request_id: 1 }, &mut out);
    let (_, bytes) = take_reply().expect("the handler replied");
    let body = &parse(&bytes).expect("reply").1[4..];
    assert_eq!(body.len(), 52);
    assert_eq!(&body[30..52], &b.driver.family.as_bytes()[..22]);
}
