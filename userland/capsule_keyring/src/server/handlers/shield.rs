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
//! The shield's custody. The shield runs the phones' wallet core, which
//! opens its account from the recovery words, so the keyring gives the words
//! of an HD wallet to the wallet that owns it, and seals them to this machine
//! and opens them again beside their account key. A wallet from a private key
//! has no words: NotFound says so, and its shield opens from the key, as on
//! the phones.

use alloc::vec::Vec;

use crate::protocol::{encode_response, Request, EACCES, EINVAL, ENOENT, ENOSPC};
use crate::store::{Store, StoreError, Words, SHIELD_MAX_WORDS};
use crate::vault::{open_seed, seal_seed, VaultError, SEED_BLOB_LEN};

const PACKED: usize = 1 + 2 * SHIELD_MAX_WORDS;

fn wipe(b: &mut [u8]) {
    for x in b.iter_mut() {
        unsafe { core::ptr::write_volatile(x, 0) };
    }
}

fn pack(words: &Words) -> [u8; PACKED] {
    let mut out = [0u8; PACKED];
    out[0] = words.count;
    for (i, w) in words.indices.iter().enumerate() {
        out[1 + 2 * i..3 + 2 * i].copy_from_slice(&w.to_le_bytes());
    }
    out
}

fn caller(req: &Request<'_>, sender_pid: u32) -> Option<(u32, u32)> {
    let p = req.payload;
    if p.len() < 8 {
        return None;
    }
    let pid = crate::server::caller::resolve_caller(
        u32::from_le_bytes([p[0], p[1], p[2], p[3]]),
        sender_pid,
    )?;
    Some((pid, u32::from_le_bytes([p[4], p[5], p[6], p[7]])))
}

fn refused(seq: u32, e: StoreError) -> Vec<u8> {
    match e {
        StoreError::AccessDenied => encode_response(seq, EACCES, &[]),
        StoreError::NotFound => encode_response(seq, ENOENT, &[]),
        _ => encode_response(seq, EINVAL, &[]),
    }
}

/// The words of wallet `id`: the count, then the indices, two bytes each.
pub fn shield_material(store: &mut Store, req: Request<'_>, sender_pid: u32) -> Vec<u8> {
    if req.payload.len() != 8 {
        return encode_response(req.seq, EINVAL, &[]);
    }
    let Some((pid, wallet)) = caller(&req, sender_pid) else {
        return encode_response(req.seq, EACCES, &[]);
    };
    let words = match store.shield_words(wallet, pid) {
        Ok(w) => w,
        Err(e) => return refused(req.seq, e),
    };
    let mut out = pack(&words);
    let used = 1 + 2 * words.count as usize;
    let resp = encode_response(req.seq, 0, &out[..used]);
    wipe(&mut out);
    resp
}

pub fn shield_seal(store: &mut Store, req: Request<'_>, sender_pid: u32) -> Vec<u8> {
    if req.payload.len() != 8 {
        return encode_response(req.seq, EINVAL, &[]);
    }
    let Some((pid, wallet)) = caller(&req, sender_pid) else {
        return encode_response(req.seq, EACCES, &[]);
    };
    let words = match store.shield_words(wallet, pid) {
        Ok(w) => w,
        Err(e) => return refused(req.seq, e),
    };
    let mut packed = pack(&words);
    let sealed = seal_seed(&packed);
    wipe(&mut packed);
    match sealed {
        Ok(blob) => encode_response(req.seq, 0, &blob),
        Err(VaultError::NoKey) => encode_response(req.seq, ENOENT, &[]),
        Err(_) => encode_response(req.seq, EINVAL, &[]),
    }
}

/// Open sealed words and keep them beside the account key `wallet`, which
/// the account record restored first.
pub fn shield_open(store: &mut Store, req: Request<'_>, sender_pid: u32) -> Vec<u8> {
    const HDR: usize = 8 + 8 + 8;
    if req.payload.len() != HDR + SEED_BLOB_LEN {
        return encode_response(req.seq, EINVAL, &[]);
    }
    let Some((pid, wallet)) = caller(&req, sender_pid) else {
        return encode_response(req.seq, EACCES, &[]);
    };
    let p = req.payload;
    let now = u64::from_le_bytes([p[8], p[9], p[10], p[11], p[12], p[13], p[14], p[15]]);
    let expires = u64::from_le_bytes([p[16], p[17], p[18], p[19], p[20], p[21], p[22], p[23]]);
    let mut key = match store.eth_secret(wallet, pid) {
        Ok(k) => k,
        Err(e) => return refused(req.seq, e),
    };
    let mut packed = match open_seed(&p[HDR..]) {
        Ok(s) => s,
        Err(e) => {
            wipe(&mut key);
            return match e {
                VaultError::NoKey => encode_response(req.seq, ENOENT, &[]),
                _ => encode_response(req.seq, EINVAL, &[]),
            };
        }
    };
    let count = (packed[0] as usize).min(SHIELD_MAX_WORDS);
    let mut indices = [0u16; SHIELD_MAX_WORDS];
    for (i, slot) in indices.iter_mut().take(count).enumerate() {
        *slot = u16::from_le_bytes([packed[1 + 2 * i], packed[2 + 2 * i]]);
    }
    wipe(&mut packed);
    // Kept only beside the account they derive: never another wallet's words.
    let theirs = crate::server::words_own::words_derive(&indices[..count], &key);
    wipe(&mut key);
    if !theirs {
        for w in indices.iter_mut() {
            unsafe { core::ptr::write_volatile(w, 0) };
        }
        return encode_response(req.seq, EINVAL, &[]);
    }
    let kept = store.put_shield_words(wallet, &indices[..count], pid, now, expires);
    for w in indices.iter_mut() {
        unsafe { core::ptr::write_volatile(w, 0) };
    }
    match kept {
        Ok(_) => encode_response(req.seq, 0, &[]),
        Err(StoreError::Full) => encode_response(req.seq, ENOSPC, &[]),
        Err(e) => refused(req.seq, e),
    }
}
