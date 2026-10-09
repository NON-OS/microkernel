// A test asserts by panicking, so the lints that forbid it are off here.
#![allow(clippy::expect_used, clippy::indexing_slicing)]

use super::rewards_vectors::{address, hex, phrase_account, M, SIGNED, T};
use super::*;

/// `linkDigest(M, M, 0)` and `linkDigest(M, T, 0)`, read from the live registry with `cast call`.
const SELF_DIGEST: &str = "0x2e8d22b51f5e710c82309b870a29e6c24df26ce88b35ec83dac1765a4169f28b";
const DIGEST: &str = "0x752ff013697fa3727eee7aa7c968bda9aeec7ddb535512145d3e74e348b6bb41";

#[test]
fn the_digest_is_what_the_registry_computes() {
    let domain = "0x36613e8b96a549e7856b58afb968064e2074577b685484ee1d5f593a561338e1";
    assert_eq!(hex(&super::typed::domain()), domain, "domainSeparator() on Sepolia");
    assert_eq!(hex(&digest(&address(M), &address(M), 0)), SELF_DIGEST);
    assert_eq!(hex(&digest(&address(M), &address(T), 0)), DIGEST);
}

#[test]
fn a_signature_is_what_foundry_makes_and_it_names_its_signer() {
    let key = phrase_account().signing_key().expect("a key");
    let made = sign_link(&key, &digest(&address(M), &address(T), 0)).expect("signed");
    assert_eq!(hex(&made), SIGNED);
    assert_eq!(signer(&digest(&address(M), &address(T), 0), &made), Some(address(M)));
    assert_ne!(signer(&digest(&address(M), &address(M), 0), &made), Some(address(M)));
}

#[test]
fn a_pasted_signature_is_held_to_what_the_registry_takes() {
    assert_eq!(parse_signature(SIGNED).map(|s| hex(&s)), Some(SIGNED.to_string()));
    let low_v = format!("{}00", &SIGNED[..130]);
    assert_eq!(parse_signature(&low_v).map(|s| s[64]), Some(27));
    assert_eq!(parse_signature(&format!("{}02", &SIGNED[..130])), None);
    assert_eq!(parse_signature(&SIGNED[..130]), None, "64 bytes");
    let high_s = "0xb686455a76f97e8e69339cf2bc2bdfdbfe9b3b4af95466edb1c0296135d00170\
ed8f5e28fb0098aff3309fbafb8be0413a5b75caeaef358f7802b5b10195cd391c";
    assert_eq!(parse_signature(high_s), None, "the malleable twin of a valid signature");
}
