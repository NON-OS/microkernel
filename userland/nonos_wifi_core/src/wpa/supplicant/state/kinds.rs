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

//! Where a handshake stands, why one failed, and what it is configured with.

use crate::wpa::akm::Akm;

/// Where the handshake stands.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum State {
    /// Waiting for message 1 (the AP's ANonce).
    Start,
    /// Message 1 seen, PTK derived, message 2 sent; awaiting message 3.
    PtkDerived,
    /// Message 3 verified, group key installed, message 4 sent. Connected; the
    /// group key handshake and message 3 retransmissions are still answered.
    Connected,
    /// Message 3 was authentic but unacceptable (its RSNE differs from the
    /// beacon's, or its key data does not hold a valid group key); the
    /// handshake is abandoned rather than trusting it.
    Failed,
}

/// Why an authentic handshake message was refused.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Failure {
    /// Message 3's RSNE or RSNXE differs from the beacon's: a downgrade attempt
    /// (IEEE Std 802.11-2020, 12.7.6.4).
    IeMismatch,
    /// The key data would not unwrap, or held no group key of the right size.
    BadKeyData,
    /// A reply could not be framed.
    Internal,
}

/// How a handshake is set up: the keys and addresses, the AKM, the elements
/// the station sent in its association request (message 2 repeats them) and
/// the access point's from its beacon (message 3 is checked against them).
pub struct Config<'a> {
    pub pmk: [u8; 32],
    pub aa: [u8; 6],
    pub spa: [u8; 6],
    pub snonce: [u8; 32],
    pub akm: Akm,
    pub own_rsne: &'a [u8],
    pub own_rsnxe: Option<&'a [u8]>,
    pub ap_rsne: &'a [u8],
    pub ap_rsnxe: Option<&'a [u8]>,
    /// Management frame protection was negotiated, so message 3 delivers the
    /// IGTK as well.
    pub pmf: bool,
}
