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

//! The empty Certificate a client declines a CertificateRequest with.

use alloc::vec::Vec;

/// A Certificate message that echoes `context` and carries no certificate,
/// which is how a client declines a CertificateRequest (RFC 8446 4.4.2).
pub(super) fn empty_certificate(context: &[u8]) -> Vec<u8> {
    let body_len = 1 + context.len() + 3;
    let mut message = Vec::with_capacity(4 + body_len);
    message.push(11);
    crate::push::u24(&mut message, body_len);
    message.push(context.len() as u8);
    message.extend_from_slice(context);
    crate::push::u24(&mut message, 0);
    message
}
