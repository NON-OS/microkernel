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

//! Why BIP-GMAC-256 refused a frame. Each refusal is its own variant so the
//! caller can count replays apart from forgeries.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BipError {
    /// Shorter than a management header, or not a management frame.
    NotManagement,
    /// BIP covers group-addressed frames only; a unicast one is CCMP or
    /// GCMP's to protect.
    NotGroupAddressed,
    /// Zero, or past the 48 bits an IPN has.
    BadIpn,
    /// The body does not end in a well-formed 16-octet-MIC MMIE.
    NoMmie,
    /// The MMIE names a key id other than the one held.
    WrongKey,
    /// The IPN is not above the last one accepted under this key.
    Replay,
    /// The MIC does not match.
    BadMic,
}
