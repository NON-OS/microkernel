//! The body a hand-off posts, from the four files a spend wrote. Every size is checked here,
//! so a malformed hand-off never leaves the phone.

use crate::error::{StoreError, WalletError};
use crate::net::relay::encode;
use crate::notes::BLOB_LEN;
use std::path::Path;

/// A v2 proof in the shared form, with its header: its size varies with the paths it shares.
const PROOF_LEN: core::ops::RangeInclusive<usize> = 80_000..=110_000;
/// Public limbs a proof carries, the not-before time last.
const LIMBS: usize = 37;

pub(super) fn read(dir: &Path, name: &str) -> Result<Vec<u8>, WalletError> {
    std::fs::read(dir.join(name)).map_err(|_| StoreError::Io.into())
}

/// The JSON for `POST /v1/handoff`, from the hand-off in `dir`.
pub(crate) fn body(dir: &Path) -> Result<String, WalletError> {
    let proof = read(dir, "spend.proof")?;
    let (payee, change) = (read(dir, "blob0.bin")?, read(dir, "blob1.bin")?);
    if !PROOF_LEN.contains(&proof.len()) || payee.len() != BLOB_LEN || change.len() != BLOB_LEN {
        return Err(StoreError::RowShape.into());
    }
    let publics = String::from_utf8(read(dir, "spend.proof.publics.json")?)
        .map_err(|_| WalletError::from(StoreError::RowShape))?;
    let limbs = limbs(&publics).ok_or(StoreError::RowShape)?;
    let list: Vec<String> = limbs.iter().map(u64::to_string).collect();
    Ok(format!(
        r#"{{"proof":"{}","publics":[{}],"blob0":"{}","blob1":"{}"}}"#,
        encode(&proof),
        list.join(","),
        encode(&payee),
        encode(&change)
    ))
}

/// The 36 limbs of `{"publics": [...]}`, each a number that fits 64 bits.
pub(super) fn limbs(text: &str) -> Option<Vec<u64>> {
    let open = text.find('[')?.checked_add(1)?;
    let close = text.find(']')?;
    let inner = text.get(open..close)?;
    let out: Vec<u64> = inner.split(',').map(|w| w.trim().parse().ok()).collect::<Option<_>>()?;
    (out.len() == LIMBS).then_some(out)
}

#[cfg(test)]
#[path = "relay_body_test.rs"]
mod relay_body_test;
