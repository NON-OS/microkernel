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
//! What a session needs from below, and what it reports.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SessionError {
    /// The client hello could not be built.
    Init,
    /// The socket failed.
    Io,
    /// The server's flight never completed.
    Handshake,
    /// The certificate chain did not verify for this host.
    Certificate,
    /// The server sent more than the caller allows.
    TooLarge,
    /*
     * The server asked for a different key exchange group. This client offers
     * x25519 alone, so there is nothing to retry with and the honest answer is to
     * say which side ended it. Left unrecognised, a retry looked exactly like a
     * peer that had gone quiet.
     */
    RetryUnsupported,
    /*
     * The peer sent an alert, and this is its description byte. It is carried
     * rather than flattened because it is the one number that says which side is
     * wrong: handshake_failure means it rejected what we offered, unknown_ca
     * means it rejected what we sent, and both used to arrive as silence.
     */
    PeerAlert(u8),
}

/// The byte stream underneath: a TCP socket, or a buffer in a test.
pub trait Io {
    fn write_all(&mut self, data: &[u8]) -> Result<(), SessionError>;
    /// Read what is available. Zero means nothing arrived this time.
    fn read(&mut self, into: &mut [u8]) -> Result<usize, SessionError>;
}
