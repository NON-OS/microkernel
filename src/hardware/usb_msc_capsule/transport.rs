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

//! One request to driver.usb_msc0 and its reply, over the shared lifecycle
//! transport, with the driver's status lifted into a client error.

use alloc::vec::Vec;
use core::sync::atomic::AtomicU32;

use spin::Mutex;

use super::error::DriverUsbMscError as E;
use super::protocol::*;
use crate::services::lifecycle::transport;

/// 0x1_0000_0013: the reply inbox the capsule's spawn registers.
pub(crate) const REPLY_INBOX: &str = "endpoint.4294967315";
const SENDER_NAME: &str = "kernel.driver_usb_msc";

static TRANSPORT_LOCK: Mutex<()> = Mutex::new(());
static SEQ: AtomicU32 = AtomicU32::new(1);

/// Send `op` with `body`; the reply's data on status 0.
pub(super) fn round_trip(op: u16, body: &[u8]) -> Result<Vec<u8>, E> {
    let _caller = super::capability::gate_call()?;
    let request_id = transport::next_request_id(&SEQ);
    let request = encode_request(op, request_id, body);
    let _guard = transport::lock_yielding(&TRANSPORT_LOCK);
    let state = crate::userspace::capsule_driver_usb_msc::shared_state();
    let resp = transport::round_trip(
        request_id,
        &request,
        SENDER_NAME,
        REPLY_INBOX,
        state,
        decode_response,
    )
    .map_err(|e| match e {
        transport::TransportError::Dead => E::Dead,
        transport::TransportError::Stale => E::Stale,
        transport::TransportError::TransportFailure => E::TransportFailure,
        transport::TransportError::ProtocolMismatch => E::ProtocolMismatch,
    })?;
    match resp.status {
        0 => Ok(resp.body),
        E_AGAIN => Err(E::NotReady),
        E_NODEV => Err(E::NoDevice),
        E_ACCES => Err(E::AccessDenied),
        E_INVAL => Err(E::InvalidArgument),
        E_NXIO => Err(E::OutOfRange),
        E_MSGSIZE => Err(E::OversizedRequest),
        E_NOTSUP => Err(E::BlockSize),
        /*
         * E_IO, and any status the driver does not document.
         */
        _ => Err(E::DeviceFailure),
    }
}
