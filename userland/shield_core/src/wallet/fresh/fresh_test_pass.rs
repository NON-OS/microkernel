//! A refund and a dispute pass to the config's Escrow, and the selectors
//! are the Escrow's own (v1.4.51).

use super::super::*;
use super::{ask, escrow, refund_data, ESCROW, FRESH, T};
use nonos_nox_config::DISPUTE_GAS;
use sha3::{Digest, Keccak256};

#[test]
fn the_selectors_are_the_escrows() {
    let sel = |sig: &str| <[u8; 4]>::try_from(&Keccak256::digest(sig)[..4]).unwrap();
    assert_eq!(sel(&format!("refund({T},address,bytes32)")), REFUND);
    assert_eq!(sel(&format!("dispute({T},uint64,bytes,bytes32,bytes32[],bytes32[])")), DISPUTE);
}

#[test]
fn a_refund_and_a_dispute_pass_to_the_config_escrow() {
    let (index, tx) = checked(&ask(refund_data(FRESH)), Some(4), escrow).unwrap();
    assert_eq!((index, tx.to, tx.value, tx.chain_id), (4, ESCROW, 0, 11_155_111));
    assert_eq!(tx.data, refund_data(FRESH));
    let dispute = ask([&DISPUTE[..], &[1; 600]].concat());
    assert!(checked(&FreshSign { gas: DISPUTE_GAS, ..dispute }, Some(4), escrow).is_ok());
}
