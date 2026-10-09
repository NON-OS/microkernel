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

//! REMOTE_NDIS_HALT_MSG (Remote NDIS 1.0, 2.2.3): sent when a bind stops
//! after the device was initialized, as Linux generic_rndis_bind does at
//! halt_fail_and_release, so the device is left uninitialized. It has no
//! completion, so nothing is read back.

use nonos_usbnet::{Bus, Setup};

use super::message::{put32, MSG_HALT, SEND_ENCAPSULATED_COMMAND};

pub(super) fn halt<B: Bus>(bus: &mut B, comm: u8) {
    let mut msg = [0u8; 12];
    put32(&mut msg, 0, MSG_HALT);
    put32(&mut msg, 4, 12);
    let send = Setup::class(false, SEND_ENCAPSULATED_COMMAND, 0, comm);
    // The bind has already failed; a device that does not take the halt
    // changes nothing about that, as Linux ignores it too.
    let _ = bus.control_out(send, &msg);
}
