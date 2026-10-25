//! The wallet's notes from the format 5 pool's three events.

#[path = "chain_scan/found.rs"]
mod found;
#[path = "chain_scan/held.rs"]
mod held;
#[path = "chain_scan/refused.rs"]
mod refused;

use nox_shield_core::custody::{generate_phrase, phrase_to_seed};
use nox_shield_core::keys::{parse_receiving_address, receiving_address_text, Account};
use nox_shield_core::notes::{commitment, seal_xwing, wire_digest, NotePlaintext, Opening};
use nox_shield_core::prover::pool_hasher;

fn account() -> Account {
    Account::from_seed(&phrase_to_seed(&generate_phrase().expect("entropy")).expect("seed"))
        .expect("account")
}

fn leaf_of(plain: &NotePlaintext) -> [u8; 32] {
    let cm = commitment(&pool_hasher(), &plain.note());
    wire_digest(&[cm[0].value(), cm[1].value(), cm[2].value(), cm[3].value()])
}

fn index_topic(i: u64) -> [u8; 32] {
    let mut t = [0u8; 32];
    t[24..].copy_from_slice(&i.to_be_bytes());
    t
}

fn abi_bytes(body: &[u8]) -> Vec<u8> {
    let mut out = vec![0u8; 64];
    out[31] = 0x20;
    out[56..64].copy_from_slice(&(body.len() as u64).to_be_bytes());
    out.extend_from_slice(body);
    while !out.len().is_multiple_of(32) {
        out.push(0);
    }
    out
}

/// A note paid to `payee` through their text address, as a settlement emits it.
fn paid_to(payee: &Account, value: u64) -> (NotePlaintext, Vec<u8>) {
    let (spend_pk, ek) =
        parse_receiving_address(&receiving_address_text(&payee.receiving_address())).expect("addr");
    let o = Opening { value, asset_id: 1, blinding: [value, 2, 3, 4] };
    let plain = NotePlaintext { value, asset_id: 1, blinding: o.blinding, spend_pk };
    let ek = x_wing::EncapsulationKey::try_from(ek.as_slice()).expect("ek");
    let blob = seal_xwing(&o, &ek, &leaf_of(&plain)).expect("seal");
    (plain, blob.to_vec())
}

const EV: [u8; 32] = [0xEE; 32];
