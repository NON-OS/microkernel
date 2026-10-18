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

//! The fixture's controller with its GET_REPORT and SET_REPORT transfers
//! made to fail or to take long: a write NACKed, or a controller slow to
//! answer.

use std::cell::Cell;
use std::rc::Rc;

use i2c_transfer_proofs::bench::{driver, GEMINI_LAKE_HZ};
use i2c_transfer_proofs::protocol::{parse, response, E_NACK, OP_TRANSFER};
use i2c_transfer_proofs::regs::{self, Attached};
use nonos_i2cmodel::{Bus, Config, Designware, HidOverI2c, Shared};
use nonos_libc::{mk_idle_ms, serve, Served};

use crate::tests::fixture::PAD;
use crate::tests::service::service;

/// Where a transfer's body holds the opcode of the HID command it writes:
/// past the address, a pad byte, the two lengths and the flags, then the
/// command register and the report's type and ID.
const OPCODE_AT: usize = 8 + 3;
const GET_REPORT: u8 = 0x02;
const SET_REPORT: u8 = 0x03;

pub struct Faulty {
    pub pad: Shared<HidOverI2c>,
    /// How many SET_REPORT transfers are still to be NACKed.
    pub nacks: Rc<Cell<u32>>,
    /// How long each SET_REPORT, and each GET_REPORT, takes past the bus.
    pub slow_ms: Rc<Cell<u64>>,
    pub slow_get_ms: Rc<Cell<u64>>,
    _served: Served,
    _attached: Attached,
}

/// `device` at PAD, where the firmware said it is, behind that controller.
pub fn faulty(device: HidOverI2c) -> Faulty {
    let pad = Shared::new(device);
    let attached = regs::attach(Designware::new(Config::LPSS, Bus::default().with(pad.clone())));
    let controller = driver(GEMINI_LAKE_HZ, PAD).expect("controller bring-up");
    let nacks = Rc::new(Cell::new(0));
    let (slow_ms, slow_get_ms) = (Rc::new(Cell::new(0)), Rc::new(Cell::new(0)));
    let (n, d, g) = (nacks.clone(), slow_ms.clone(), slow_get_ms.clone());
    let served = serve(move |req, resp| match parse(req) {
        Some((r, body)) if r.op == OP_TRANSFER => match body.get(OPCODE_AT) {
            Some(&SET_REPORT) if n.get() > 0 => {
                n.set(n.get() - 1);
                response(r.op, r.request_id, E_NACK, &[], resp) as i64
            }
            Some(&op) if op == SET_REPORT || op == GET_REPORT => {
                mk_idle_ms(if op == SET_REPORT { d.get() } else { g.get() });
                service(&controller, req, resp)
            }
            _ => service(&controller, req, resp),
        },
        _ => service(&controller, req, resp),
    });
    Faulty { pad, nacks, slow_ms, slow_get_ms, _served: served, _attached: attached }
}
