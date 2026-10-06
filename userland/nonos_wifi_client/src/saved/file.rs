/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The record on disk: `[magic 4][nonce 12][ChaCha20-Poly1305 of the slots]
//! [tag 16]`, always the same length. All zeros is the withdrawn record.
//!
//! The NONOS store is written to disk as it is given (vfs store at LBA 256,
//! no encryption of its own), so the slots never reach it unsealed. The magic
//! and the path are the associated data: a record moved to another name or
//! with its header changed does not open.

use nonos_seal::{open, seal, NONCE_LEN, TAG_LEN};

use super::error::SavedError;
use super::list::{SavedList, PLAIN_LEN};
use crate::wipe::wipe;

pub(super) const DIR: &[u8] = b"/nonos/wifi";
pub(super) const PATH: &[u8] = b"/nonos/wifi/saved";
const MAGIC: &[u8; 4] = b"NWF1";
const AAD: &[u8] = b"NWF1/nonos/wifi/saved";
const HEAD: usize = MAGIC.len() + NONCE_LEN;
pub(super) const FILE_LEN: usize = HEAD + PLAIN_LEN + TAG_LEN;

/// Seal `list` under `key` with `nonce` into `out`. False only if the cipher
/// refused, which a correctly sized buffer never makes it do.
pub(super) fn seal_file(
    list: &SavedList,
    key: &[u8; 32],
    nonce: &[u8; NONCE_LEN],
    out: &mut [u8; FILE_LEN],
) -> bool {
    let mut plain = [0u8; PLAIN_LEN];
    list.encode(&mut plain);
    out[..MAGIC.len()].copy_from_slice(MAGIC);
    out[MAGIC.len()..HEAD].copy_from_slice(nonce);
    let sealed = seal(key, nonce, AAD, &plain, &mut out[HEAD..]);
    wipe(&mut plain);
    sealed.is_ok()
}

/// Open a sealed record read back from the store.
pub(super) fn open_file(raw: &[u8], key: &[u8; 32]) -> Result<SavedList, SavedError> {
    if raw.len() != FILE_LEN || raw[..MAGIC.len()] != MAGIC[..] {
        return Err(SavedError::Damaged);
    }
    let mut nonce = [0u8; NONCE_LEN];
    nonce.copy_from_slice(&raw[MAGIC.len()..HEAD]);
    let mut plain = [0u8; PLAIN_LEN];
    let opened = open(key, &nonce, AAD, &raw[HEAD..], &mut plain);
    let list = match opened {
        Ok(PLAIN_LEN) => SavedList::decode(&plain).ok_or(SavedError::Damaged),
        Ok(_) => Err(SavedError::Damaged),
        Err(_) => Err(SavedError::Unreadable),
    };
    wipe(&mut plain);
    list
}

/// True for the withdrawn record, which needs no key to read.
pub(super) fn is_withdrawn(raw: &[u8]) -> bool {
    raw.iter().all(|b| *b == 0)
}
