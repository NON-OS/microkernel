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

use alloc::vec::Vec;

use super::constants::{EXT_KEY_SHARE, GROUP_SECP256R1, GROUP_X25519};

/*
 * One share per offered group, in the order ext_groups names them, so a
 * server that takes either answers at once with no HelloRetryRequest. The
 * P-256 share is the 65-byte uncompressed point RFC 8446 4.2.8.2 requires.
 */
pub fn ext_keyshare(out: &mut Vec<u8>, public: &[u8; 32], p256: &[u8; 65]) {
    let mut body = Vec::with_capacity(107);
    super::push::u16(&mut body, (4 + public.len() + 4 + p256.len()) as u16);
    super::push::u16(&mut body, GROUP_X25519);
    super::push::u16(&mut body, public.len() as u16);
    body.extend_from_slice(public);
    super::push::u16(&mut body, GROUP_SECP256R1);
    super::push::u16(&mut body, p256.len() as u16);
    body.extend_from_slice(p256);
    super::push::ext(out, EXT_KEY_SHARE, &body);
}
