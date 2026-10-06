// A test asserts by panicking, so the lints that forbid it are off here.
#![allow(clippy::indexing_slicing)]

use super::rewards_vectors::{address, bytes, hex, M, SIGNED, T};
use super::*;

/// Pinned against `cast calldata "link(address,bytes,bool)" M SIGNED false`.
#[test]
fn the_link_call_is_what_foundry_encodes() {
    let call = link_calldata(&address(M), &bytes(SIGNED), false);
    let head = "0xc7d350d10000000000000000000000009858effd232b4033e47d90003d41ec34ecaeda94\
0000000000000000000000000000000000000000000000000000000000000060\
0000000000000000000000000000000000000000000000000000000000000000\
0000000000000000000000000000000000000000000000000000000000000041";
    assert_eq!(hex(&call[..4 + 4 * 32]), head);
    assert_eq!(&call[4 + 4 * 32..4 + 4 * 32 + 65], bytes(SIGNED).as_slice());
    assert_eq!(call.len(), 4 + 4 * 32 + 96);
    assert_eq!(hex(&unlink_calldata()), "0x565a8a3e");
}

/// The text `cast wallet sign --data` signed to `SIGNED`, field by field.
#[test]
fn the_typed_data_is_the_text_foundry_signed() {
    let json = typed_json(&address(M), &address(T), 0);
    let domain = r#"[{"name":"name","type":"string"},{"name":"version","type":"string"},"#;
    let link = r#"[{"name":"mainnet","type":"address"},{"name":"testnet","type":"address"},"#;
    let types = format!(
        r#"{{"types":{{"EIP712Domain":{domain}{{"name":"chainId","type":"uint256"}}],"Link":{link}"#
    );
    assert!(json.starts_with(&types));
    let rest = r#"{"name":"nonce","type":"uint256"}]},"primaryType":"Link","#;
    let domain_value = r#""domain":{"name":"NOX testnet rewards","version":"1","chainId":1},"#;
    let message = format!(r#""message":{{"mainnet":"{M}","testnet":"{T}","nonce":"0"}}}}"#);
    assert_eq!(json, format!("{types}{rest}{domain_value}{message}"));
}
