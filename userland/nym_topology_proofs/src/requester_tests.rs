// NONOS Operating System (AGPL-3.0-or-later)
//! Exit discovery from the validator's described nodes, over a slice of the
//! live answer (vectors/SOURCE.txt says which).

use crate::directory_sync::api::base58::decode32;
use crate::directory_sync::requesters::{parse_address, parse_described};

const DESCRIBED: &[u8] = include_bytes!("../vectors/described.json");
const EXITS: &str = include_str!("../vectors/exits.txt");

fn exits() -> Vec<[u8; 32]> {
    EXITS.lines().map(|l| decode32(l.trim().as_bytes()).expect("exit id")).collect()
}

#[test]
fn every_requester_on_a_listed_exit_is_kept() {
    let ex = exits();
    let found = parse_described(DESCRIBED, |g| ex.contains(g));
    assert_eq!(found.len(), 3);
    for (f, e) in found.iter().zip(ex.iter()) {
        assert_eq!(&f.gateway, e, "the gateway is the exit that published it");
    }
}

#[test]
fn a_requester_whose_gateway_is_not_a_listed_exit_is_dropped() {
    let ex = exits();
    let all = parse_described(DESCRIBED, |_| true);
    let kept = parse_described(DESCRIBED, |g| ex.contains(g));
    // Four addresses parse; the fourth names a gateway outside the exit list.
    assert_eq!(all.len(), 4);
    assert_eq!(kept.len(), 3);
    assert!(!ex.contains(&all[3].gateway));
}

#[test]
fn an_address_is_identity_dot_encryption_at_gateway() {
    let a = parse_address(
        b"Ht4gVegGguGvdw74hXumfCMEFNNHAVyLiSSraWFnv4y3.8QEMqgxcHr2XFp51fcptpveV3pBVArvBsyE18kL9GzEb@CXcCVGiamYSwgVwaxW3mEkXkZh1sKY2TXnWjjTjxDxzA",
    )
    .expect("a live address parses");
    assert_eq!(a.gateway, decode32(b"CXcCVGiamYSwgVwaxW3mEkXkZh1sKY2TXnWjjTjxDxzA").unwrap());
    assert_eq!(a.identity, decode32(b"Ht4gVegGguGvdw74hXumfCMEFNNHAVyLiSSraWFnv4y3").unwrap());
    for bad in [&b"no-at-sign"[..], b"a@b", b".@", b"x.y@", b"@", b""] {
        assert!(parse_address(bad).is_none(), "{:?}", core::str::from_utf8(bad));
    }
}

#[test]
fn a_document_with_no_nodes_yields_no_exits() {
    assert!(parse_described(b"{\"data\":[]}", |_| true).is_empty());
    assert!(parse_described(b"not json at all", |_| true).is_empty());
}
