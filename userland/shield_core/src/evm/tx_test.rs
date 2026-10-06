/*
 * A test asserts by panicking, so the lints that forbid it are off here.
 */
#![allow(clippy::unwrap_used, clippy::indexing_slicing, clippy::arithmetic_side_effects)]

//! Two transactions signed here and by Foundry's `cast mktx` with the same
//! key, the public "abandon ... about" test account, byte for byte.

use super::Eip1559;
use crate::evm::account::test_account;

fn to_hex(bytes: &[u8]) -> String {
    let body: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
    format!("0x{body}")
}

fn dead() -> [u8; 20] {
    let mut to = [0u8; 20];
    to[18..].copy_from_slice(&[0xde, 0xad]);
    to
}

#[test]
fn an_ether_transfer_on_mainnet_matches_foundry() {
    let tx = Eip1559 {
        chain_id: 1,
        nonce: 7,
        max_priority_fee: 1_500_000_000,
        max_fee: 30_000_000_000,
        gas: 21_000,
        to: dead(),
        value: 1_000_000_000_000_000,
        data: Vec::new(),
    };
    let signed = tx.sign(&test_account().signing_key().unwrap()).unwrap();
    assert_eq!(to_hex(&signed.raw), "0x02f87201078459682f008506fc23ac0082520894000000000000000000000000000000000000dead87038d7ea4c6800080c001a00ad65a45776c928f387a756f442bdb1adfe1d20f306a6a9ed7251af2ec09587ea01ab93298e6b8b3170c131faed3c1c081b3136f610b5fb655fdc8072add97608a");
}

#[test]
fn a_token_transfer_on_sepolia_matches_foundry() {
    let mut data = vec![0xa9, 0x05, 0x9c, 0xbb];
    data.extend_from_slice(&[0u8; 12]);
    data.extend_from_slice(&dead());
    data.extend_from_slice(&[0u8; 16]);
    data.extend_from_slice(&1_000_000_000_000_000_000u128.to_be_bytes());
    let tx = Eip1559 {
        chain_id: 11_155_111,
        nonce: 0,
        max_priority_fee: 1_000_000_000,
        max_fee: 20_000_000_000,
        gas: 65_000,
        to: *b"\x3e\x52\x49\xa6\x5c\xa5\x13\xd5\xe1\x12\x60\x22\x2e\x0d\x26\xf4\x6b\x46\x5d\x36",
        value: 0,
        data,
    };
    let signed = tx.sign(&test_account().signing_key().unwrap()).unwrap();
    assert_eq!(to_hex(&signed.raw), "0x02f8b383aa36a780843b9aca008504a817c80082fde8943e5249a65ca513d5e11260222e0d26f46b465d3680b844a9059cbb000000000000000000000000000000000000000000000000000000000000dead0000000000000000000000000000000000000000000000000de0b6b3a7640000c080a02fa0210938a86c1b6d7ee320f743143c4334cb87ff4de545fbe2309a44e0ad91a04b135f868b7ca38e7958a28089e98f0b46b477cfdc5b83e80c260680cdfa7488");
}

#[test]
fn the_same_transaction_on_the_other_network_is_a_different_signature() {
    let key = test_account().signing_key().unwrap();
    let base = Eip1559 {
        chain_id: 1,
        nonce: 0,
        max_priority_fee: 1,
        max_fee: 2,
        gas: 21_000,
        to: dead(),
        value: 1,
        data: Vec::new(),
    };
    let other = Eip1559 { chain_id: 11_155_111, ..base.clone() };
    assert_ne!(base.sign(&key).unwrap().hash, other.sign(&key).unwrap().hash);
}
