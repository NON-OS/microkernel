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

//! A further account of an HD wallet: account `index` of the same words, as
//! every Ethereum wallet numbers them (m/44'/60'/0'/0/index). The words never
//! leave the keyring for this; they are read beside the root account, which
//! must be the caller's and unlocked, and the new account keeps them too, so
//! the shield opens it from the same phrase and deleting it deletes them.
//!
//! Payload: caller pid (4) | root account id (4) | index (4) | now (8) |
//! expires_at (8). Answer: the new account's id (4).

use alloc::vec::Vec;

use nonos_hd::wipe;

use crate::protocol::{encode_response, Request, EACCES, EINVAL, ENOENT, ENOSPC};
use crate::store::{eth_secret_valid, KeyType, Store, StoreError};

/// Accounts 1 to 7 beside the root: each account holds two entries (its key
/// and its words), so eight of them are an owner's whole share of sixteen.

pub fn wallet_derive(store: &mut Store, req: Request<'_>, sender_pid: u32) -> Vec<u8> {
    let p = req.payload;
    if p.len() != 28 {
        return encode_response(req.seq, EINVAL, &[]);
    }
    let le32 = |at: usize| u32::from_le_bytes([p[at], p[at + 1], p[at + 2], p[at + 3]]);
    let le64 = |at: usize| {
        let mut b = [0u8; 8];
        b.copy_from_slice(&p[at..at + 8]);
        u64::from_le_bytes(b)
    };
    let Some(pid) = super::super::caller::resolve_caller(le32(0), sender_pid) else {
        return encode_response(req.seq, EACCES, &[]);
    };
    let (root, index, now, expires_at) = (le32(4), le32(8), le64(12), le64(20));
    // Account 0 is the root itself.
    if index == 0 || index > super::super::words_own::MAX_ACCOUNT_INDEX {
        return encode_response(req.seq, EINVAL, &[]);
    }
    let words = match store.shield_words(root, pid) {
        Ok(w) => w,
        Err(StoreError::AccessDenied) => return encode_response(req.seq, EACCES, &[]),
        Err(StoreError::NotFound) => return encode_response(req.seq, ENOENT, &[]),
        Err(_) => return encode_response(req.seq, EINVAL, &[]),
    };
    let used = &words.indices[..(words.count as usize).min(words.indices.len())];
    let Some(mut key) = super::super::hd::account_key_at(used, index) else {
        return encode_response(req.seq, EINVAL, &[]);
    };
    if !eth_secret_valid(&key) {
        wipe(&mut key);
        return encode_response(req.seq, EINVAL, &[]);
    }
    let stored = store.store(KeyType::Secp256k1Eth, &key, pid, now, expires_at);
    wipe(&mut key);
    let result = match stored {
        Ok(id) if super::super::hd::keep_seed(store, id, used, pid, now, expires_at) => Ok(id),
        Ok(_) => Err(StoreError::Full),
        Err(e) => Err(e),
    };
    match result {
        Ok(id) => encode_response(req.seq, 0, &id.to_le_bytes()),
        Err(StoreError::Full) => encode_response(req.seq, ENOSPC, &[]),
        Err(_) => encode_response(req.seq, EINVAL, &[]),
    }
}
