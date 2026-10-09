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

//! What a completed server flight yields.

extern crate alloc;

use alloc::vec::Vec;

use crate::traffic_keys::TrafficKeys;

pub struct ServerComplete {
    pub handshake: TrafficKeys,
    pub app: TrafficKeys,
    /// Hash of ClientHello through the server Finished, which the client
    /// Finished and the application keys are both computed over.
    pub transcript_hash: [u8; 32],
    /// The Certificate message body, so a caller that authenticates the peer
    /// itself can reach the leaf. Empty if none arrived.
    pub certificates: Vec<u8>,
    /// The empty Certificate message the client owes when the server sent a
    /// CertificateRequest; `None` when it did not.
    pub client_certificate: Option<Vec<u8>>,
    /// Hash of ClientHello through the client's own Certificate when it owes
    /// one, else the same as `transcript_hash`. The client Finished covers
    /// this; the application keys never do.
    pub finished_hash: [u8; 32],
}
