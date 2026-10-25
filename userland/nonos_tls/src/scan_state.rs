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

//! What the scan of the server's flight carries from message to message.

use alloc::vec::Vec;

use super::transcript::Transcript;

pub struct ScanState<'a> {
    pub secret: &'a [u8; 32],
    pub transcript: &'a mut Transcript,
    pub host: &'a [u8],
    pub now: u64,
    pub cert11: &'a mut Vec<u8>,
    pub validated: &'a mut bool,
    /// Whether the certificate has to chain to a trusted root for this host.
    ///
    /// True for every ordinary connection. False only where the caller
    /// authenticates the peer itself, which it must then actually do: the
    /// session is otherwise bound to nothing but whoever answered.
    pub require_chain: bool,
    /// The certificate_request_context of a CertificateRequest, when the
    /// server sent one. The client must then answer with a Certificate of its
    /// own, empty here, before its Finished (RFC 8446 section 4.4.2).
    pub request_context: &'a mut Option<Vec<u8>>,
}
