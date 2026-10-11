// NONOS Operating System (AGPL-3.0-or-later)
//! SIGN_FROM_FRESH's fields and the shield's callers (v1.4.51).
use super::*;
use alloc::vec::Vec;

fn ask() -> FreshAsk {
    FreshAsk {
        address: [0xf1; 20],
        chain_id: 11_155_111,
        nonce: 3,
        gas: 150_000,
        max_fee: 20_000_000_000,
        max_priority: 1_000_000_000,
        to: [0xe5; 20],
        value: 0,
        data: Vec::from([0xa3, 0x81, 0x4a, 0x14, 0x00, 0xff]),
    }
}

#[test]
fn a_fresh_sign_request_reads_back_through_the_frame() {
    let a = ask();
    let f = a.fields();
    let words: Vec<&str> = f.iter().map(|s| s.as_str()).collect();
    let bytes = encode_request(1, OP_SIGN_FROM_FRESH, &words);
    let r = decode_request(&bytes).unwrap();
    let back: Vec<&str> = r.body.split('\n').collect();
    assert_eq!((r.op, FreshAsk::parse(&back)), (17, Some(a)));
    assert_eq!(FreshAsk::parse(&back[..8]), None, "a field short");
    let mut bad = back.clone();
    bad[8] = "0xa38";
    assert_eq!(FreshAsk::parse(&bad), None, "odd hex");
    bad[8] = "a3814a14";
    assert_eq!(FreshAsk::parse(&bad), None, "no 0x");
    bad = back.clone();
    bad[0] = "0x1234";
    assert_eq!(FreshAsk::parse(&bad), None, "not 20 bytes");
}

#[test]
fn only_a_wallet_window_may_call() {
    let pid_of = |name: &[u8]| match name {
        b"app.nonos_wallet" => Some(40),
        b"app.nonos_wallet.2" => Some(42),
        b"app.store" => Some(50),
        _ => None,
    };
    assert!(from_wallet(40, pid_of) && from_wallet(42, pid_of));
    assert!(!from_wallet(50, pid_of), "the store");
    assert!(!from_wallet(41, pid_of), "a name nobody holds");
    assert!(!from_wallet(0, |_| Some(0)), "no sender");
}
