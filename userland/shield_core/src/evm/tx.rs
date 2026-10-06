//! An EIP-1559 transaction, its signature, and the bytes that are broadcast.
//! The chain id is signed, so a transaction for one network fails on every
//! other. The RFC 6979 signature with a low s is what the chain accepts, and
//! it makes the encoding testable against Foundry's byte for byte.

use super::rlp;
use k256::ecdsa::SigningKey;
use sha3::{Digest, Keccak256};

/// The fields of a type 2 transaction, with an empty access list.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Eip1559 {
    pub chain_id: u64,
    pub nonce: u64,
    pub max_priority_fee: u128,
    pub max_fee: u128,
    pub gas: u64,
    pub to: [u8; 20],
    pub value: u128,
    pub data: Vec<u8>,
}

/// A signed transaction: what is broadcast, and the hash the chain knows it by.
pub struct Signed {
    pub raw: Vec<u8>,
    pub hash: [u8; 32],
}

impl Eip1559 {
    fn fields(&self) -> Vec<u8> {
        let mut out = Vec::new();
        rlp::uint(&mut out, self.chain_id.into());
        rlp::uint(&mut out, self.nonce.into());
        rlp::uint(&mut out, self.max_priority_fee);
        rlp::uint(&mut out, self.max_fee);
        rlp::uint(&mut out, self.gas.into());
        rlp::bytes(&mut out, &self.to);
        rlp::uint(&mut out, self.value);
        rlp::bytes(&mut out, &self.data);
        rlp::list(&mut out, &[]);
        out
    }

    /// Type byte then the list, the form both signature and broadcast take.
    fn typed(items: &[u8]) -> Vec<u8> {
        let mut out = vec![0x02];
        rlp::list(&mut out, items);
        out
    }

    /// Sign with `key`, adding only the recovery bit, r and s, so the review a
    /// screen showed is the transaction sent.
    pub fn sign(&self, key: &SigningKey) -> Option<Signed> {
        let digest: [u8; 32] = Keccak256::digest(Self::typed(&self.fields())).into();
        let (signature, recovery) = key.sign_prehash_recoverable(&digest).ok()?;
        let (r, s) = signature.split_bytes();
        let mut items = self.fields();
        rlp::uint(&mut items, u128::from(recovery.is_y_odd()));
        rlp::word(&mut items, &r.into());
        rlp::word(&mut items, &s.into());
        let raw = Self::typed(&items);
        let hash = Keccak256::digest(&raw).into();
        Some(Signed { raw, hash })
    }
}

#[cfg(test)]
#[path = "tx_test.rs"]
mod tx_test;
