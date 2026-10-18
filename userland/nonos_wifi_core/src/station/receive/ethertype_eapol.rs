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

//! The receive half of an associated link: what the access point sent, checked
//! before anything reaches the IP stack or the supplicant.
//!
//! A data frame is taken only from the BSS (FromDS, transmitter the BSSID) to
//! this station or a group address. Fragments and A-MSDUs are refused (this
//! station negotiates neither, and both are what the FragAttacks injections
//! ride on). On a protected link an unprotected frame is refused unless it is
//! EAPOL (a repeated message 3 arrives in the clear before the AP installs the
//! pairwise key); every protected frame is decrypted, by the chip (which
//! leaves the CCMP header and MIC in place) or here under the pairwise or
//! group key its key index names, and must pass the per-TID replay check.
//! EAPOL goes to the supplicant, which answers the group key handshake; the
//! rest is converted to Ethernet for the stack.

pub(super) const ETHERTYPE_EAPOL: [u8; 2] = [0x88, 0x8E];

pub(super) const FC_MORE_FRAGMENTS: u16 = 0x0400;

/// The A-MSDU Present bit of the QoS Control field.
pub(super) const QOS_AMSDU: u8 = 0x80;

/// Data subtypes that carry no payload (Null, QoS Null and the CF ones).
pub(super) const SUBTYPE_NO_DATA: u8 = 0x04;

/// Why a received frame was not delivered.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RxDrop {
    NotAssociated,
    /// Not a data frame with a payload, or shorter than its headers.
    Malformed,
    /// Not FromDS from the BSSID.
    NotFromBss,
    /// Addressed to another station.
    NotForUs,
    Fragmented,
    Amsdu,
    /// An unprotected frame that is not EAPOL on a protected link.
    Unprotected,
    /// No key to decrypt it under, or the MIC failed.
    Undecryptable,
    Replay,
    /// The body is not LLC/SNAP encapsulated.
    NotLlc,
    /// EAPOL arrived with no supplicant to answer it.
    NoSupplicant,
    /// A management frame with nothing for the stack: a beacon, a probe
    /// response, another network's, or one the link does not act on.
    Management,
}
