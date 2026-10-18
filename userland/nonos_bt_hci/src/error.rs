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

//! Why the host stopped.

use nonos_bt_wire::hci::opcode::Opcode;
use nonos_bt_wire::WireError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HciError {
    /// A packet from the controller was malformed.
    Wire(WireError),
    /// The USB controller driver refused a transfer with this errno.
    Transport(i32),
    /// No answer to this command within the poll budget.
    Timeout(Opcode),
    /// The controller answered this command with a non-zero status
    /// (Vol 1, Part F lists the codes).
    Status(Opcode, u8),
    /// The controller does not support LE (Vol 2, Part C, 3.3).
    NoLe,
    /// No free ACL buffer on the controller; wait for Number of Completed
    /// Packets (Vol 4, Part E, 4.1.1).
    NoCredit,
    /// The controller reports no ACL buffers, or bring-up has not run.
    NoBuffers,
    /// No Bluetooth interface in the device's configuration.
    NotBluetooth,
    /// The controller lost its state and must be brought up again: it
    /// reported a Hardware Error (Vol 4, Part E, 7.7.16), completed a Reset
    /// (7.3.2) this host did not send, or no longer knows a link it never
    /// reported down.
    Reset,
    /// A Three-wire UART link never became active, or its peer stopped
    /// acknowledging what was sent (Vol 4, Part D).
    NoLink,
}

impl From<WireError> for HciError {
    fn from(e: WireError) -> Self {
        Self::Wire(e)
    }
}
