//! A fresh address's signature recovers to it, and only the index handed
//! out signs (v1.4.51), its SHIELD_FROM_FRESH sends too (v1.4.56).

use super::super::*;
use crate::evm::tx::Eip1559;
use k256::ecdsa::{RecoveryId, Signature, VerifyingKey};
use sha3::{Digest, Keccak256};

use super::rlp::items;
use super::shield::{approve, tx, POOL, TOKEN};
use nonos_nox_config::APPROVE_GAS;

pub(super) fn seed() -> [u8; 64] {
    let mut words = [0u16; 12];
    words[11] = 3;
    let mut seed = [0u8; 64];
    assert!(nonos_hd::bip39::seed_from_words(&words, b"", &mut seed));
    seed
}

pub(super) fn address_at(index: u32) -> [u8; 20] {
    crate::evm::EvmAccount::at(&seed(), index).unwrap().address()
}

#[test]
fn the_signature_recovers_to_the_fresh_address() {
    let fresh = address_at(5);
    let tx = Eip1559 {
        chain_id: 11_155_111,
        nonce: 2,
        max_priority_fee: 1_000_000_000,
        max_fee: 20_000_000_000,
        gas: 150_000,
        to: [0xE5; 20],
        value: 0,
        data: vec![0xa3, 0x81, 0x4a, 0x14, 1, 2, 3],
    };
    let signed = sign_at(&seed(), 5, &fresh, &tx).unwrap();
    assert_eq!(signed.raw[0], 0x02, "type 2");
    assert_eq!(signed.hash, <[u8; 32]>::from(Keccak256::digest(&signed.raw)));
    assert_eq!((items(&signed.raw).len(), items(&signed.raw)[8].len()), (12, 0), "v r s");
    assert_eq!(recovered(&signed.raw, &tx), fresh);
    assert!(sign_at(&seed(), 4, &fresh, &tx).is_none(), "another index's key never signs");
}

/// The address a signed type-2 transaction's v, r and s recover to.
pub(super) fn recovered(raw: &[u8], tx: &Eip1559) -> [u8; 20] {
    let it = items(raw);
    let word = |b: &[u8]| {
        let mut w = [0u8; 32];
        w[32 - b.len()..].copy_from_slice(b);
        w
    };
    let sig = Signature::from_scalars(word(&it[10]), word(&it[11])).unwrap();
    let v = RecoveryId::from_byte(it[9].first().copied().unwrap_or(0)).unwrap();
    let key = VerifyingKey::recover_from_prehash(&tx.digest(), &sig, v).unwrap();
    let point = key.to_encoded_point(false);
    Keccak256::digest(&point.as_bytes()[1..])[12..].try_into().unwrap()
}

#[test]
fn the_approval_and_the_deposits_recover_to_the_handed_address() {
    let fresh = address_at(7);
    for t in [tx(TOKEN, approve(5), APPROVE_GAS), tx(POOL, vec![0xde, 0xad], SHIELD_DEPOSIT_GAS)] {
        let signed = sign_at(&seed(), 7, &fresh, &t).unwrap();
        assert_eq!(recovered(&signed.raw, &t), fresh);
        assert!(sign_at(&seed(), 8, &fresh, &t).is_none(), "another index never signs");
    }
}
