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

//! The controller capsule's service, as the HID driver's calls reach it: the
//! shipping handler for transfers, the device setup bound for the ACPI
//! query, a refusal for everything else.

use i2c_transfer_proofs::driver::Driver;
use i2c_transfer_proofs::protocol::{
    parse, response, E_BAD_OP, E_INVAL, E_OK, IPC_PAYLOAD_MAX, OP_ACPI_HID, OP_TRANSFER,
};
use i2c_transfer_proofs::server::handlers::transfer::handle;
use nonos_libc::{take_reply, SERVICE_PID};

/// Reply length into `resp`, or the errno the kernel would return.
pub fn service(controller: &Driver, req: &[u8], resp: &mut [u8]) -> i64 {
    let Some((request, body)) = parse(req) else { return E_INVAL as i64 };
    let mut out = [0u8; IPC_PAYLOAD_MAX];
    let reply = match request.op {
        OP_TRANSFER => {
            handle(controller, SERVICE_PID, &request, body, &mut out);
            take_reply().map(|(_, bytes)| bytes).unwrap_or_default()
        }
        OP_ACPI_HID => {
            let n =
                response(request.op, request.request_id, E_OK, &acpi_body(controller), &mut out);
            out[..n].to_vec()
        }
        _ => {
            let n = response(request.op, request.request_id, E_BAD_OP, &[], &mut out);
            out[..n].to_vec()
        }
    };
    if reply.len() > resp.len() {
        return E_INVAL as i64;
    }
    resp[..reply.len()].copy_from_slice(&reply);
    reply.len() as i64
}

/// What the shipping OP_ACPI_HID handler answers for a controller setup
/// bound: address, descriptor register and interrupt pin as little-endian
/// halves, then the HID_INFO bits; nothing when setup bound no device and
/// the firmware declared no address.
fn acpi_body(c: &Driver) -> Vec<u8> {
    if c.bound_addr == 0 {
        return Vec::new();
    }
    let halves = [u16::from(c.bound_addr), c.bound_desc_reg, c.bound_pin].map(u16::to_le_bytes);
    [halves.concat(), vec![c.bound_info, 0]].concat()
}
