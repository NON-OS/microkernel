// NONOS Operating System (AGPL-3.0-or-later)
/* The frames a caller sends, and the handshake most tests begin with. */

use alloc::vec::Vec;

use super::socks::front::Front;
use super::socks_fake::Fake;

pub const PID: u32 = 40;
pub const HELLO: [u8; 3] = [5, 1, 0];

pub fn bytes(frame: &[u8]) -> Vec<u8> {
    [&[0u8][..], frame].concat()
}

pub fn numbered(seq: u32, frame: &[u8]) -> Vec<u8> {
    [&[2u8][..], &seq.to_le_bytes(), frame].concat()
}

pub fn connect_example() -> Vec<u8> {
    [&[5u8, 1, 0, 3, 11][..], b"example.com", &443u16.to_be_bytes()].concat()
}

/// Greet and ask for example.com:443, returning the answer to the request.
pub fn handshake(front: &mut Front, t: &mut Fake) -> Vec<u8> {
    assert_eq!(front.serve(t, PID, &bytes(&HELLO)), [0, 5, 0]);
    front.serve(t, PID, &bytes(&connect_example()))
}
