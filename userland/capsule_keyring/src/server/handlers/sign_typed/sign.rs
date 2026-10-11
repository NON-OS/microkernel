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

//! OP_SIGN_TYPED_DATA: an EIP-712 signature over one of the messages in the
//! keyring's own table (server/typed), for a key the caller owns.
//!
//! The caller names an entry and gives its field values. The type, the
//! domain and the verifying contract come from the table, so no request
//! can turn this into a Permit or an approval signed blind.

use alloc::vec::Vec;

use super::errno::errno;
use crate::protocol::{encode_response, Request, EACCES, EINVAL};
use crate::server::ethaddr::address_of;
use crate::server::registry::owner_of;
use crate::server::typed::{check, hashes::hashes, refusal::Refusal, request, table};
use crate::server::zeroize::zeroize32;
use crate::store::{Store, StoreError};

pub fn sign_typed(store: &mut Store, req: Request<'_>, sender_pid: u32) -> Vec<u8> {
    match sign(store, req.payload, sender_pid) {
        Ok(out) => encode_response(req.seq, 0, &out),
        Err(errno) => encode_response(req.seq, errno, &[]),
    }
}

fn sign(store: &mut Store, p: &[u8], sender_pid: u32) -> Result<Vec<u8>, i32> {
    let t = request::parse(p).ok_or(errno(Refusal::Malformed))?;
    let e = table::entry(t.entry).ok_or(errno(Refusal::UnknownEntry))?;
    let pid = check::caller_allowed(e, sender_pid, owner_of)
        .then(|| crate::server::caller::resolve_caller(t.pid, sender_pid))
        .flatten()
        .ok_or(errno(Refusal::Caller))?;
    let contract = check::admit(e, t.chain, t.words).map_err(errno)?;
    let mut secret = store.eth_secret(t.key, pid).map_err(|e| match e {
        StoreError::AccessDenied => EACCES,
        _ => EINVAL,
    })?;
    let signed = address_of(&secret).ok_or(EINVAL).and_then(|signer| {
        check::signer_rule(e, t.words, &signer).map_err(errno)?;
        let h = hashes(e, t.chain, &contract, t.words, keccak).ok_or(EINVAL)?;
        let mut sig = [0u8; 65];
        if crate::server::secp::sign(&secret, &h.digest, &mut sig) != 65 {
            return Err(EINVAL);
        }
        Ok([&signer[..], &h.domain_separator, &h.struct_hash, &h.digest, &sig].concat())
    });
    zeroize32(&mut secret);
    signed
}

fn keccak(b: &[u8]) -> Option<[u8; 32]> {
    let mut out = [0u8; 32];
    (nonos_libc::crypto_keccak256(b.as_ptr(), b.len(), out.as_mut_ptr(), 32) == 32).then_some(out)
}
