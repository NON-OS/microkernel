// NONOS Operating System (AGPL-3.0-or-later)
/* Replies that do not fit the stage the build is in, or whose contents do
 * not hold up, fail by name rather than bringing a hop up. */

use crate::circuit::build::answer::answer;
use crate::circuit::build::error::BuildError;
use crate::circuit::Hop;

use super::circuit_mirror::{backward, created2, keys, mirror};
use crate::cell::RELAY_EXTENDED2;

const REPLY: [u8; 64] = [0xA5; 64];

#[test]
fn the_wrong_kind_of_reply_for_the_stage_is_a_protocol_error() {
    let mut relays = [mirror(&keys(1))];
    let mut relay_cell = backward(&mut relays, 0, RELAY_EXTENDED2, &REPLY);
    assert_eq!(answer(&mut relay_cell, &mut [], |_| None).err(), Some(BuildError::Protocol));
    let mut hops = [Hop::new(&keys(1))];
    let got = answer(&mut created2(&REPLY), &mut hops, |_| None);
    assert_eq!(got.err(), Some(BuildError::Protocol), "no CREATED2 once a hop is up");
}

#[test]
fn a_handshake_that_does_not_verify_is_named() {
    let got = answer(&mut created2(&REPLY), &mut [], |_| None);
    assert_eq!(got.err(), Some(BuildError::Handshake));
}

#[test]
fn a_reply_longer_than_the_cell_is_a_protocol_error() {
    let mut cell = created2(&REPLY);
    cell.payload[..2].copy_from_slice(&600u16.to_be_bytes());
    assert_eq!(answer(&mut cell, &mut [], |_| None).err(), Some(BuildError::Protocol));
}
