//! Every custody error, named as the shells' tables name it.

use nox_shield_core::error::CustodyError;

fn named_custody(error: &CustodyError) -> &'static str {
    match error {
        CustodyError::Entropy => "entropy",
        CustodyError::Mnemonic => "mnemonic",
        CustodyError::PhraseLength => "phrase length",
        CustodyError::SealAuth => "seal auth",
        CustodyError::Guard => "guard",
        CustodyError::VaultShape => "vault shape",
        CustodyError::VaultPresent => "vault present",
        CustodyError::Locked => "locked",
        CustodyError::KeyShape => "key shape",
    }
}
#[test]
fn every_custody_error_has_a_name_a_shell_can_look_up() {
    let all = [
        CustodyError::Entropy,
        CustodyError::Mnemonic,
        CustodyError::PhraseLength,
        CustodyError::SealAuth,
        CustodyError::Guard,
        CustodyError::VaultShape,
        CustodyError::VaultPresent,
        CustodyError::Locked,
        CustodyError::KeyShape,
    ];
    for error in &all {
        assert!(!named_custody(error).is_empty(), "a variant reached the shells unnamed");
    }
}
