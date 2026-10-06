// A test asserts by panicking, so the lints that forbid it are off here.
#![allow(clippy::expect_used, clippy::indexing_slicing)]

use crate::evm::EvmAccount;

/// The first two accounts of the public test phrase, "abandon" eleven times then "about".
pub(super) const M: &str = "0x9858EfFD232B4033E47d90003D41EC34EcaEda94";
pub(super) const T: &str = "0x6Fac4D18c912343BF86fa7049364Dd4E424Ab9C0";
/// `cast wallet sign --data` with account 0 over `typed_json(M, T, 0)`.
pub(super) const SIGNED: &str =
    "0xebcd8b57af4c850392b3f03827a7c9f4fcaa984d1e71e54d0181abad3ade9804\
501bc294ce36981d850b1018f0932bfde8e8544b3717f6ed830520796b5e74ce1b";

pub(super) fn bytes(text: &str) -> Vec<u8> {
    crate::net::rpc::hex_bytes(text).expect("hex")
}

pub(super) fn hex(data: &[u8]) -> String {
    let body: String = data.iter().map(|b| format!("{b:02x}")).collect();
    format!("0x{body}")
}

pub(super) fn address(text: &str) -> [u8; 20] {
    bytes(text).as_slice().try_into().expect("an address")
}

pub(super) fn phrase_account() -> EvmAccount {
    let mut words = [0u16; 12];
    words[11] = 3;
    let mut seed = [0u8; 64];
    assert!(nonos_hd::bip39::seed_from_words(&words, b"", &mut seed));
    EvmAccount::from_seed(&seed).expect("the account")
}
