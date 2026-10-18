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

//! The fixture's controller with the pad's interrupt line wired to its
//! doorbell (OP_GPIO_DOORBELL): asserted while the pad has a report queued,
//! as an I2C-HID device holds it, or while the test holds it asserted. A
//! pad just powered up may refuse its first writes: SET_REPORTs NACKed.

use std::cell::Cell;
use std::rc::Rc;

use i2c_transfer_proofs::bench::{driver, GEMINI_LAKE_HZ};
use i2c_transfer_proofs::protocol::OP_TRANSFER;
use i2c_transfer_proofs::protocol::{parse, response, E_NACK, E_OK, OP_GPIO_DOORBELL};
use i2c_transfer_proofs::regs::{self, Attached};
use nonos_i2cmodel::{Bus, Config, Designware, HidOverI2c, Shared};
use nonos_libc::{serve, Served};

use super::fixture::PAD;
use super::service::service;

pub struct Wired {
    pub pad: Shared<HidOverI2c>,
    /// The line held asserted whatever the pad has queued.
    pub held: Rc<Cell<bool>>,
    /// How many SET_REPORT transfers are still to be NACKed.
    pub nacks: Rc<Cell<u32>>,
    _served: Served,
    _attached: Attached,
}

pub fn wired(device: HidOverI2c) -> Wired {
    let pad = Shared::new(device);
    let attached = regs::attach(Designware::new(Config::LPSS, Bus::default().with(pad.clone())));
    let controller = driver(GEMINI_LAKE_HZ, PAD).expect("controller bring-up");
    let (held, nacks) = (Rc::new(Cell::new(false)), Rc::new(Cell::new(0)));
    let (line, queue, refusals) = (held.clone(), pad.clone(), nacks.clone());
    let served = serve(move |req, resp| match parse(req) {
        Some((request, _)) if request.op == OP_GPIO_DOORBELL => {
            let fired = line.get() || queue.lock().pending_inputs() > 0;
            let body = [1u32.to_le_bytes(), u32::from(fired).to_le_bytes()].concat();
            response(request.op, request.request_id, E_OK, &body, resp) as i64
        }
        // A SET_REPORT's opcode is the fourth byte it writes (faulty.rs).
        Some((request, body))
            if request.op == OP_TRANSFER && body.get(11) == Some(&0x03) && refusals.get() > 0 =>
        {
            refusals.set(refusals.get() - 1);
            response(request.op, request.request_id, E_NACK, &[], resp) as i64
        }
        _ => service(&controller, req, resp),
    });
    Wired { pad, held, nacks, _served: served, _attached: attached }
}
