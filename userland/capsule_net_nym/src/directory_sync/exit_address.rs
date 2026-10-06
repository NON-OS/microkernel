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

//! A network requester's address, and reading one from its text form.

use super::api::base58::decode32;

/// The exit that opens connections on our behalf.
#[derive(Clone, Copy)]
pub struct ExitAddress {
    pub identity: [u8; 32],
    pub encryption: [u8; 32],
    pub gateway: [u8; 32],
}

/// Parse `identity.encryption@gateway`, each part a base58 key of 32 bytes.
pub fn parse_address(text: &[u8]) -> Option<ExitAddress> {
    let at = text.iter().position(|&b| b == b'@')?;
    let (client, gateway) = (&text[..at], &text[at + 1..]);
    let dot = client.iter().position(|&b| b == b'.')?;
    Some(ExitAddress {
        identity: decode32(&client[..dot])?,
        encryption: decode32(&client[dot + 1..])?,
        gateway: decode32(gateway)?,
    })
}
