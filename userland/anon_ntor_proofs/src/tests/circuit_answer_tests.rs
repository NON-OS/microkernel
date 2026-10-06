// NONOS Operating System (AGPL-3.0-or-later)
/* A hop comes up only from the reply its own handshake is waiting on, sent
 * by the hop that was asked, and every other reply is named as a failure. */

use crate::cell::{Cell, CELL_DESTROY, RELAY_EXTENDED2, RELAY_TRUNCATED};
use crate::circuit::build::answer::answer;
use crate::circuit::build::error::BuildError;
use crate::circuit::Hop;

use super::circuit_mirror::{backward, created2, keys, mirror, ID};

const REPLY: [u8; 64] = [0xA5; 64];

fn expect(reply: &[u8]) -> impl FnOnce(&[u8]) -> Option<[u8; 72]> + '_ {
    move |got| (got == reply).then(|| keys(9))
}

#[test]
fn a_created2_brings_up_the_first_hop_from_its_own_reply() {
    assert!(answer(&mut created2(&REPLY), &mut [], expect(&REPLY)).is_ok());
}

#[test]
fn an_extended2_from_the_last_hop_brings_up_the_next() {
    let mut hops = [Hop::new(&keys(1))];
    let mut relays = [mirror(&keys(1))];
    let mut cell = backward(&mut relays, 0, RELAY_EXTENDED2, &REPLY);
    assert!(answer(&mut cell, &mut hops, expect(&REPLY)).is_ok());
}

#[test]
fn an_extended2_from_an_earlier_hop_is_refused() {
    let mut hops = [Hop::new(&keys(1)), Hop::new(&keys(2))];
    let mut relays = [mirror(&keys(1)), mirror(&keys(2))];
    let mut cell = backward(&mut relays, 0, RELAY_EXTENDED2, &REPLY);
    let got = answer(&mut cell, &mut hops, expect(&REPLY));
    assert_eq!(got.err(), Some(BuildError::Protocol), "only the hop asked may answer");
}

#[test]
fn truncated_and_destroy_are_named_as_destroyed() {
    let mut hops = [Hop::new(&keys(1))];
    let mut relays = [mirror(&keys(1))];
    let mut cell = backward(&mut relays, 0, RELAY_TRUNCATED, &[]);
    assert_eq!(answer(&mut cell, &mut hops, |_| None).err(), Some(BuildError::Destroyed));
    let mut destroy = Cell::new(ID, CELL_DESTROY);
    assert_eq!(answer(&mut destroy, &mut [], |_| None).err(), Some(BuildError::Destroyed));
}

#[test]
fn a_cell_no_hop_sealed_is_unrecognised() {
    let mut hops = [Hop::new(&keys(1))];
    let mut relays = [mirror(&keys(4))];
    let mut cell = backward(&mut relays, 0, RELAY_EXTENDED2, &REPLY);
    assert_eq!(answer(&mut cell, &mut hops, |_| None).err(), Some(BuildError::Unrecognised));
}
