// NONOS Operating System (AGPL-3.0-or-later)
/* Greeting and CONNECT: what the front answers before any byte is relayed. */

use alloc::vec::Vec;

use super::socks::frame::{ask, is_socks, Ask};
use super::socks::front::Front;
use super::socks::tunnel::Far;
use super::socks_fake::fake;
use super::socks_frames::{bytes, handshake, HELLO, PID};

#[test]
fn a_socks_frame_never_reads_as_an_api_request() {
    /* The API magic 0x414E4F31, little endian, opens with 0x31. */
    assert!(!is_socks(&0x414E_4F31u32.to_le_bytes()));
    assert!(is_socks(&[0]) && is_socks(&[1]) && is_socks(&[2, 0, 0, 0, 0]));
    assert!(!is_socks(&[]));
    assert_eq!(ask(&[2, 9, 0, 0, 0, 0xAB]), Some(Ask::Numbered(9, &[0xAB][..])));
    assert_eq!(ask(&[2, 9, 0]), None, "a numbered frame too short to carry its number");
}

#[test]
fn success_waits_for_the_exit_to_connect() {
    let (mut front, mut t) = (Front::default(), fake());
    assert_eq!(handshake(&mut front, &mut t), [0], "nothing is promised while the exit dials");
    assert_eq!(t.opened, [(b"example.com".to_vec(), 443)], "the name goes to the exit unresolved");
    assert_eq!(front.serve(&mut t, PID, &bytes(&[])), [0]);
    t.far = Far::Open;
    assert_eq!(front.serve(&mut t, PID, &bytes(&[])), [0, 5, 0, 0, 1, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn a_greeting_split_across_frames_is_joined() {
    let (mut front, mut t) = (Front::default(), fake());
    assert_eq!(front.serve(&mut t, PID, &bytes(&[5])), [0]);
    assert_eq!(front.serve(&mut t, PID, &bytes(&[1, 0])), [0, 5, 0]);
}

#[test]
fn a_caller_offering_no_open_method_is_refused_and_closed() {
    let (mut front, mut t) = (Front::default(), fake());
    assert_eq!(front.serve(&mut t, PID, &bytes(&[5, 1, 2])), [1, 5, 0xFF]);
}

#[test]
fn an_ipv6_destination_is_refused_by_address_type() {
    let (mut front, mut t) = (Front::default(), fake());
    front.serve(&mut t, PID, &bytes(&HELLO));
    let req: Vec<u8> = [&[5u8, 1, 0, 4][..], &[0; 16], &443u16.to_be_bytes()].concat();
    assert_eq!(front.serve(&mut t, PID, &bytes(&req)), [1, 5, 8, 0, 1, 0, 0, 0, 0, 0, 0]);
    assert!(t.opened.is_empty());
}

#[test]
fn an_ipv4_destination_reaches_the_exit_as_text() {
    let (mut front, mut t) = (Front::default(), fake());
    front.serve(&mut t, PID, &bytes(&HELLO));
    front.serve(&mut t, PID, &bytes(&[5, 1, 0, 1, 93, 184, 215, 14, 0, 80]));
    assert_eq!(t.opened, [(b"93.184.215.14".to_vec(), 80)]);
}

#[test]
fn a_network_not_ready_is_answered_as_unreachable() {
    let (mut front, mut t) = (Front::default(), fake());
    t.refuse = Some(3);
    assert_eq!(handshake(&mut front, &mut t), [1, 5, 3, 0, 1, 0, 0, 0, 0, 0, 0]);
}
