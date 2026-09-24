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

//! Unframing an RSA verify request; the scheme itself lives next door.

use alloc::vec::Vec;

use super::rsa_scheme::verify;
use crate::protocol::{encode_response, Request, EBADMSG, EINVAL, OP_RSA_VERIFY};

pub fn rsa_verify(req: Request<'_>) -> Vec<u8> {
    let reply = |status: i32| encode_response(OP_RSA_VERIFY, req.flags, req.request_id, status, &[]);
    let p = req.payload;
    if p.len() < 6 {
        return reply(EINVAL);
    }
    let scheme = p[0];
    let hashid = p[1];
    let spki_len = u16::from_le_bytes([p[2], p[3]]) as usize;
    let mut o = 4;
    if p.len() < o + spki_len + 2 {
        return reply(EINVAL);
    }
    let spki = &p[o..o + spki_len];
    o += spki_len;
    let sig_len = u16::from_le_bytes([p[o], p[o + 1]]) as usize;
    o += 2;
    if p.len() < o + sig_len {
        return reply(EINVAL);
    }
    let sig = &p[o..o + sig_len];
    o += sig_len;
    match verify(scheme, hashid, spki, sig, &p[o..]) {
        Some(true) => reply(0),
        Some(false) => reply(EBADMSG),
        /*
         * A key that will not decode, a digest of the wrong length, or a
         * scheme that is not offered. None of those is a failed signature, and
         * reporting them as one would tell a caller its key was rejected.
         */
        None => reply(EINVAL),
    }
}
