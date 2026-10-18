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

//! A client's OP_TRANSFER as driver.i2c_hid0 sends it, through the shipping
//! handler, and a bus with the touchpad and a GPIO expander on it.

use nonos_i2cmodel::{Bus, Config, Designware, Expander, HidOverI2c, Shared};
use nonos_libc::take_reply;

use crate::driver::Driver;
use crate::protocol::{parse, Request, IPC_PAYLOAD_MAX, OP_TRANSFER};
use crate::regs::{self, Attached};
use crate::server::handlers::transfer;
use crate::transaction::FLAG_RESTART_ON_READ;

/// A PCA9555-type GPIO expander's usual address.
pub const EXPANDER: u8 = 0x20;

/// The errno the handler answers a transfer to `addr` that writes `write`,
/// then reads `read_len` bytes after a repeated START.
pub fn transfer_errno(driver: &Driver, addr: u8, write: &[u8], read_len: u16) -> i32 {
    let mut body = vec![addr, 0];
    body.extend((write.len() as u16).to_le_bytes());
    body.extend(read_len.to_le_bytes());
    body.extend(FLAG_RESTART_ON_READ.to_le_bytes());
    body.extend_from_slice(write);
    let req = Request { op: OP_TRANSFER, request_id: 7 };
    let mut out = [0u8; IPC_PAYLOAD_MAX];
    transfer::handle(driver, 42, &req, &body, &mut out);
    let (_, bytes) = take_reply().expect("the handler replied");
    let (_, payload) = parse(&bytes).expect("reply header parses");
    i32::from_le_bytes(payload[0..4].try_into().expect("errno"))
}

/// `pad` and an expander at EXPANDER on one modelled bus.
pub fn with_expander(pad: HidOverI2c) -> (Shared<HidOverI2c>, Shared<Expander>, Attached) {
    let (pad, expander) = (Shared::new(pad), Shared::new(Expander::new(EXPANDER)));
    let bus = Bus::default().with(pad.clone()).with(expander.clone());
    (pad, expander, regs::attach(Designware::new(Config::LPSS, bus)))
}
