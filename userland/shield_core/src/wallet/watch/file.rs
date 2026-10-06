//! The view keys this wallet was given, sealed in one file under a key derived from the seed.
//! A view key shows every note of an account, so it is kept like the notes are: never in the
//! clear on disk. Each write draws a fresh nonce and replaces the file whole.

use crate::entropy::fill;
use crate::error::StoreError;
use chacha20poly1305::{AeadInOut, ChaCha20Poly1305, KeyInit};
use std::path::Path;
use zeroize::Zeroizing;

const AAD: &[u8] = b"nox-shield/watching/v1";
const NONCE: usize = 12;
const TAG: usize = 16;

/// The view key texts in the file, or none when there is no file.
pub(crate) fn read(path: &Path, key: &[u8; 32]) -> Result<Vec<Zeroizing<String>>, StoreError> {
    let blob = match std::fs::read(path) {
        Ok(b) => b,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(_) => return Err(StoreError::Io),
    };
    let (nonce, rest) = blob.split_at_checked(NONCE).ok_or(StoreError::RowAuth)?;
    let (body, tag) = rest
        .split_at_checked(rest.len().checked_sub(TAG).ok_or(StoreError::RowAuth)?)
        .ok_or(StoreError::RowAuth)?;
    let mut plain = Zeroizing::new(body.to_vec());
    let nonce: [u8; NONCE] = nonce.try_into().map_err(|_| StoreError::RowAuth)?;
    let tag: [u8; TAG] = tag.try_into().map_err(|_| StoreError::RowAuth)?;
    ChaCha20Poly1305::new(&(*key).into())
        .decrypt_inout_detached(&nonce.into(), AAD, plain.as_mut_slice().into(), &tag.into())
        .map_err(|_| StoreError::RowAuth)?;
    let text = std::str::from_utf8(&plain).map_err(|_| StoreError::RowAuth)?;
    Ok(text.lines().map(|l| Zeroizing::new(l.to_string())).collect())
}

/// Seal `keys` and replace the file with them, through a side file.
pub(crate) fn write(path: &Path, key: &[u8; 32], keys: &[&str]) -> Result<(), StoreError> {
    let mut nonce = [0u8; NONCE];
    fill(&mut nonce).map_err(|_| StoreError::Io)?;
    let mut body = Zeroizing::new(keys.join("\n").into_bytes());
    let tag = ChaCha20Poly1305::new(&(*key).into())
        .encrypt_inout_detached(&nonce.into(), AAD, body.as_mut_slice().into())
        .map_err(|_| StoreError::Io)?;
    let mut blob = nonce.to_vec();
    blob.extend_from_slice(&body);
    blob.extend_from_slice(&tag);
    let side = path.with_extension("new");
    std::fs::write(&side, blob).map_err(|_| StoreError::Io)?;
    std::fs::rename(side, path).map_err(|_| StoreError::Io)
}
