//! A fresh address's signature recovers to it, and only the index handed
//! out signs (v1.4.51).

use super::super::*;
use crate::evm::tx::Eip1559;
use k256::ecdsa::{RecoveryId, Signature, VerifyingKey};
use sha3::{Digest, Keccak256};

use super::rlp::items;

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
    let it = items(&signed.raw);
    assert_eq!((it.len(), it[8].len()), (12, 0), "nine fields, an empty access list, v r s");
    let word = |b: &[u8]| {
        let mut w = [0u8; 32];
        w[32 - b.len()..].copy_from_slice(b);
        w
    };
    let sig = Signature::from_scalars(word(&it[10]), word(&it[11])).unwrap();
    let v = RecoveryId::from_byte(it[9].first().copied().unwrap_or(0)).unwrap();
    let key = VerifyingKey::recover_from_prehash(&tx.digest(), &sig, v).unwrap();
    let point = key.to_encoded_point(false);
    let got = &Keccak256::digest(&point.as_bytes()[1..])[12..];
    assert_eq!(got, &fresh[..]);
    assert!(sign_at(&seed(), 4, &fresh, &tx).is_none(), "another index's key never signs");
}
