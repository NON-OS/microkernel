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

//! What the pairing state machine asks of its caller after each PDU: PDUs
//! to send on the SMP channel, something to show or ask the user, and how
//! pairing ended or what it waits on.

use alloc::vec::Vec;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum User {
    /// Show these six digits; the peer's user types them.
    ShowPasskey(u32),
    /// Ask the user for the six digits the peer shows.
    EnterPasskey,
    /// Ask the user whether both sides show these six digits.
    ConfirmNumber(u32),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bond {
    pub ltk: [u8; 16],
    /// The peer's identity resolving key, if it sent one.
    pub irk: Option<[u8; 16]>,
    /// The peer's identity address (type, address in wire order).
    pub identity: Option<(u8, [u8; 6])>,
    /// False for Just Works, which no user step protects from a device in
    /// the middle (Vol 3, Part H, 2.3.5.2).
    pub authenticated: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// Start encryption with this LTK (EDIV and Rand zero, Vol 3, Part H,
    /// 2.4.4), then call `encrypted`.
    Encrypt([u8; 16]),
    Paired(Bond),
    /// Pairing ended with this reason; `by_peer` if the peer sent it.
    Failed {
        reason: u8,
        by_peer: bool,
    },
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Step {
    pub send: Vec<Vec<u8>>,
    pub user: Option<User>,
    pub outcome: Option<Outcome>,
}
