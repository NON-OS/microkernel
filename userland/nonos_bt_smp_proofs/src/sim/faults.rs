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

//! The ways the simulated responder can be told to misbehave.

#[derive(Default)]
pub struct Faults {
    /// Public key to send instead of ours, in air order.
    pub key: Option<[u8; 64]>,
    /// Send back the initiator's own public key.
    pub reflect: bool,
    pub bad_confirm: bool,
    pub bad_check: bool,
    /// Identity address before the IRK.
    pub keys_reversed: bool,
}
