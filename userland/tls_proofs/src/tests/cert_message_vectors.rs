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

//! Building a Certificate message out of captured certificates.

pub(crate) fn message(certs: &[&[u8]]) -> Vec<u8> {
    let mut list = Vec::new();
    for der in certs {
        list.extend_from_slice(&[(der.len() >> 16) as u8, (der.len() >> 8) as u8, der.len() as u8]);
        list.extend_from_slice(der);
        list.extend_from_slice(&[0x00, 0x00]); // no extensions
    }
    let mut body = vec![0u8]; // empty certificate_request_context
    body.extend_from_slice(&[(list.len() >> 16) as u8, (list.len() >> 8) as u8, list.len() as u8]);
    body.extend_from_slice(&list);
    body
}
