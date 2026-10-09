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


//! Why a TLS 1.2 handshake or session ended, one reason each.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tls12Error {
    /// The socket failed.
    Io,
    /// The server went quiet mid-handshake.
    Quiet,
    /// The server sent more than this client reads.
    TooLarge,
    /// The server sent this alert.
    PeerAlert(u8),
    /// A record or message did not parse.
    Malformed,
    /// A message arrived out of order, or one that has no place here.
    Unexpected,
    /// The server picked something this client did not offer.
    Unoffered,
    /// The server's random carries the TLS 1.3 downgrade mark.
    Downgrade,
    /// The ServerKeyExchange signature does not verify under the
    /// certificate's key.
    Signature,
    /// The server's Finished does not match the transcript.
    Finished,
    /// Randomness, key agreement or the AEAD call failed.
    Crypto,
}

impl Tls12Error {
    /// The reason, for the serial log.
    pub fn said(self) -> &'static [u8] {
        match self {
            Self::Io => b"tls12 socket failed",
            Self::Quiet => b"tls12 server went quiet",
            Self::TooLarge => b"tls12 server sent too much",
            Self::PeerAlert(_) => b"tls12 server sent an alert",
            Self::Malformed => b"tls12 record or message malformed",
            Self::Unexpected => b"tls12 message out of order",
            Self::Unoffered => b"tls12 server chose what was not offered",
            Self::Downgrade => b"tls12 refused: server random carries the tls13 downgrade mark",
            Self::Signature => b"tls12 server key exchange signature does not verify",
            Self::Finished => b"tls12 server finished does not match",
            Self::Crypto => b"tls12 crypto call failed",
        }
    }
}
