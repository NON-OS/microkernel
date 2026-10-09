//! The public account key of one account index, and its address.

use k256::ecdsa::SigningKey;
use sha3::{Digest, Keccak256};
use zeroize::Zeroizing;

pub struct EvmAccount {
    key: Zeroizing<[u8; 32]>,
    address: [u8; 20],
}

impl EvmAccount {
    /// The account at m/44'/60'/0'/0/0 of a BIP-39 seed, which every Ethereum wallet shows first.
    pub fn from_seed(seed: &[u8; 64]) -> Option<EvmAccount> {
        EvmAccount::at(seed, 0)
    }

    /// The account at m/44'/60'/0'/0/`index`, or nothing if a step landed off the curve.
    pub fn at(seed: &[u8; 64], index: u32) -> Option<EvmAccount> {
        EvmAccount::of_key(super::derive::key_at(seed, index, uncompressed)?)
    }

    /// The account a private key brought in controls.
    pub fn from_key(key: &[u8; 32]) -> Option<EvmAccount> {
        EvmAccount::of_key(Zeroizing::new(*key))
    }

    fn of_key(key: Zeroizing<[u8; 32]>) -> Option<EvmAccount> {
        let public = uncompressed(&key)?;
        let digest = Keccak256::digest(public.get(1..)?);
        let mut address = [0u8; 20];
        address.copy_from_slice(digest.get(12..)?);
        Some(EvmAccount { key, address })
    }

    pub fn address(&self) -> [u8; 20] {
        self.address
    }

    /// The private key's bytes, for the domain separation test only.
    #[cfg(test)]
    pub(crate) fn secret(&self) -> [u8; 32] {
        *self.key
    }

    /// The signing key, for the transaction builder in this module only.
    pub(crate) fn signing_key(&self) -> Option<SigningKey> {
        SigningKey::from_bytes(self.key.as_ref().into()).ok()
    }
}

fn uncompressed(secret: &[u8; 32]) -> Option<[u8; 65]> {
    let key = SigningKey::from_bytes(secret.into()).ok()?;
    let point = key.verifying_key().to_encoded_point(false);
    <[u8; 65]>::try_from(point.as_bytes()).ok()
}

#[path = "account_export.rs"]
mod export;

#[cfg(test)]
#[path = "account_fixture.rs"]
mod fixture;
#[cfg(test)]
pub(crate) use fixture::test_account;

#[cfg(test)]
#[path = "account_test.rs"]
mod account_test;
