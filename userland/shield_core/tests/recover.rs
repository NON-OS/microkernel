//! Claim, refund and root publication, pinned against Foundry's encoder.
#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

use nox_shield_core::wallet::{beta_refund_calldata, claim_calldata, commit_root_calldata};

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

const WHO: [u8; 20] = [0x42; 20];
const ARGS: &str = "0000000000000000000000000000000000000000000000000000000000000001\
                    0000000000000000000000004242424242424242424242424242424242424242";

#[test]
fn each_call_is_what_foundry_encodes() {
    assert_eq!(hex(&claim_calldata(1, &WHO)), format!("881a1ce0{ARGS}"));
    assert_eq!(hex(&beta_refund_calldata(1, &WHO)), format!("f9a8cd1d{ARGS}"));
    assert_eq!(hex(&commit_root_calldata()), "d34353c9");
}
