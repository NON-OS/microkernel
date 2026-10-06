//! Reading a pasted address. Whitespace and case are forgiven, a failed checksum never is,
//! so a mistyped address cannot resolve to somebody else's account.

use super::{checksum, Address, CHECK, PAYLOAD, PREFIX};
use crate::error::WalletError;
use crate::keys::base32_read::decode;

impl Address {
    /// Parse an address a user pasted.
    pub fn parse(text: &str) -> Result<Address, WalletError> {
        let cleaned = text.trim().to_lowercase();
        let body = cleaned.strip_prefix(PREFIX).ok_or(WalletError::Address)?;
        let bytes = decode(body).ok_or(WalletError::Address)?;
        if bytes.len() != PAYLOAD + CHECK {
            return Err(WalletError::Address);
        }
        let (payload, check) = bytes.split_at(PAYLOAD);
        if checksum(payload)[..] != *check {
            return Err(WalletError::Address);
        }
        let (keys, view) = payload.split_at_checked(32).ok_or(WalletError::Address)?;
        let mut spend_pk = [0u64; 4];
        for (slot, chunk) in spend_pk.iter_mut().zip(keys.chunks_exact(8)) {
            let mut word = [0u8; 8];
            word.copy_from_slice(chunk);
            *slot = u64::from_le_bytes(word);
        }
        Ok(Address { spend_pk, view_pk: view.try_into().map_err(|_| WalletError::Address)? })
    }
}
