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

//! The controller service of the fixture, keeping a log of what the HID
//! driver asked it for, and able to answer OP_ACPI_HID with what a
//! misbehaving peer might say instead of what setup bound.

use std::cell::RefCell;
use std::rc::Rc;

use i2c_transfer_proofs::bench::{driver, unaddressed, GEMINI_LAKE_HZ};
use i2c_transfer_proofs::protocol::{parse, response, E_OK, OP_ACPI_HID, OP_TRANSFER};
use i2c_transfer_proofs::regs::{self, Attached};
use nonos_i2cmodel::{Bus, Config, Designware, HidOverI2c, Shared};
use nonos_libc::{serve, Served};

use super::service::service;

/// The ops the HID driver called, in order, with the address of each
/// transfer.
pub type Calls = Rc<RefCell<Vec<(u16, Option<u8>)>>>;

pub struct Recorded {
    pub pad: Shared<HidOverI2c>,
    pub calls: Calls,
    _served: Served,
    _attached: Attached,
}

/// `device` on the bus, the controller bound as `hint` says (None: by its
/// sweep), and OP_ACPI_HID answered with `acpi` when given.
pub fn recorded(device: HidOverI2c, hint: Option<u8>, acpi: Option<[u8; 8]>) -> Recorded {
    let pad = Shared::new(device);
    let attached = regs::attach(Designware::new(Config::LPSS, Bus::default().with(pad.clone())));
    let controller = match hint {
        Some(addr) => driver(GEMINI_LAKE_HZ, addr),
        None => unaddressed(GEMINI_LAKE_HZ),
    };
    let controller = controller.expect("controller bring-up");
    let calls = Calls::default();
    let log = calls.clone();
    let served = serve(move |req, resp| {
        let Some((request, body)) = parse(req) else { return service(&controller, req, resp) };
        let addr = (request.op == OP_TRANSFER).then(|| body.first().copied()).flatten();
        log.borrow_mut().push((request.op, addr));
        match acpi.filter(|_| request.op == OP_ACPI_HID) {
            Some(answer) => {
                let n = response(request.op, request.request_id, E_OK, &answer, resp);
                n as i64
            }
            None => service(&controller, req, resp),
        }
    });
    Recorded { pad, calls, _served: served, _attached: attached }
}
