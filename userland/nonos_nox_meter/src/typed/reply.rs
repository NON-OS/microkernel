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

//! The 181-byte reply: signer[20] | domainSeparator[32] | structHash[32] |
//! digest[32] | sig[65], with v biased by 27 as op 11 returns it.

pub const REPLY_LEN: usize = 181;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Signed {
    pub signer: [u8; 20],
    pub domain_separator: [u8; 32],
    pub struct_hash: [u8; 32],
    pub digest: [u8; 32],
    pub sig: [u8; 65],
}

pub fn parse_reply(b: &[u8]) -> Option<Signed> {
    if b.len() != REPLY_LEN {
        return None;
    }
    Some(Signed {
        signer: b[..20].try_into().ok()?,
        domain_separator: b[20..52].try_into().ok()?,
        struct_hash: b[52..84].try_into().ok()?,
        digest: b[84..116].try_into().ok()?,
        sig: b[116..].try_into().ok()?,
    })
}
