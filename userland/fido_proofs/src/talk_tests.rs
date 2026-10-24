// NONOS Operating System (AGPL-3.0-or-later)
//! Requests go out as whole 64-byte reports; a channel is asked for on the
//! broadcast channel and taken only from the answer with our nonce.

use nonos_usbnet::mock::{Call, MockBus};

use crate::ctaphid::{allocate, send, CMD_CBOR};
use crate::keyside::{first, init_answer};

fn bus() -> MockBus {
    MockBus::new(Box::new(|_| Ok(vec![])))
}

fn sent(b: &MockBus) -> Vec<Vec<u8>> {
    let calls = b.0.borrow().calls.clone();
    calls
        .into_iter()
        .filter_map(|c| if let Call::InterruptOut(2, p) = c { Some(p) } else { None })
        .collect()
}

#[test]
fn a_request_is_its_first_report_then_continuations() {
    let mut b = bus();
    let cbor: Vec<u8> = (0..100).collect();
    send(&mut b, 2, [1, 2, 3, 4], CMD_CBOR, &cbor).unwrap();
    let p = sent(&b);
    assert_eq!(p.len(), 2);
    assert_eq!(p[0][..7], [1, 2, 3, 4, 0x90, 0, 100]);
    assert_eq!(p[0][7..], cbor[..57]);
    assert_eq!(p[1][..5], [1, 2, 3, 4, 0]);
    assert_eq!(p[1][5..5 + 43], cbor[57..]);
    assert!(p.iter().all(|r| r.len() == 64) && p[1][5 + 43..].iter().all(|&x| x == 0));
}

#[test]
fn a_request_past_the_framing_is_refused_unsent() {
    let mut b = bus();
    assert_eq!(send(&mut b, 2, [1; 4], CMD_CBOR, &vec![0; 7610]), Err(-22));
    assert!(sent(&b).is_empty());
    send(&mut b, 2, [1; 4], CMD_CBOR, &vec![0; 7609]).unwrap();
    assert_eq!(sent(&b).len(), 129, "57 + 128 * 59 bytes");
}

#[test]
fn the_channel_comes_from_the_answer_with_our_nonce() {
    let mut b = bus();
    let nonce = [9, 8, 7, 6, 5, 4, 3, 2];
    let notices = [
        init_answer([0; 8], [5; 4], 0x04),
        first([7; 4], 0x90, 1, &[0]),
        init_answer(nonce, [0xA, 0xB, 0xC, 0xD], 0x0D),
    ];
    b.0.borrow_mut().notices.extend(notices);
    let ch = allocate(&mut b, (3, 2), nonce, || true).unwrap();
    assert_eq!(
        (ch.cid, ch.protocol, ch.version, ch.caps),
        ([0xA, 0xB, 0xC, 0xD], 2, [5, 2, 7], 0x0D)
    );
    assert_eq!(sent(&b)[0][..15], [0xFF, 0xFF, 0xFF, 0xFF, 0x86, 0, 8, 9, 8, 7, 6, 5, 4, 3, 2]);
}

#[test]
fn a_broadcast_channel_is_no_channel_and_silence_runs_out() {
    let mut b = bus();
    let nonce = [1; 8];
    b.0.borrow_mut().notices.push_back(init_answer(nonce, [0xFF; 4], 0x04));
    let mut looks = 0;
    let wait = || {
        looks += 1;
        looks <= 5
    };
    assert_eq!(allocate(&mut b, (3, 2), nonce, wait).map(|c| c.cid), Err(-110));
}
