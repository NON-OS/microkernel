// NONOS Operating System (AGPL-3.0-or-later)
/* A conversation net.anon no longer holds is said to be lost, and a status
 * ask says how far the network has got. Before, a restarted net.anon read a
 * caller's stream bytes as a SOCKS greeting and closed, which the browser
 * could only report as the exit hanging up; and a cold one refused every
 * CONNECT with no word of how far its bootstrap was. */

use super::socks::front::Front;
use super::socks::tunnel::{Far, Progress};
use super::socks_fake::fake;
use super::socks_frames::{handshake, numbered, PID};

const LOST: u8 = 2;
const STATUS: u8 = 3;
const STATUS_ASK: u8 = 6;

fn on(stream: u32, seq: u32, body: &[u8]) -> Vec<u8> {
    [&[4u8][..], &stream.to_le_bytes(), &seq.to_le_bytes(), body].concat()
}

#[test]
fn a_later_exchange_of_a_conversation_never_held_is_lost() {
    let (mut front, mut t) = (Front::default(), fake());
    assert_eq!(front.serve(&mut t, PID, &on(3, 7, &[0x17, 3, 3])), [LOST]);
    assert_eq!(front.serve(&mut t, PID, &numbered(4, &[])), [LOST], "stream 0 too");
    assert!(t.opened.is_empty() && t.sent.is_empty(), "nothing reached the network");
}

#[test]
fn the_first_exchange_still_begins_a_conversation() {
    let (mut front, mut t) = (Front::default(), fake());
    assert_eq!(front.serve(&mut t, PID, &on(3, 1, &[5, 1, 0])), [0, 5, 0]);
    assert_eq!(front.serve(&mut t, PID, &on(3, 2, &[])), [0], "held, so carried on");
}

#[test]
fn an_ended_conversation_gives_back_its_close_not_lost() {
    let (mut front, mut t) = (Front::default(), fake());
    assert_eq!(front.serve(&mut t, PID, &on(2, 1, &[5, 1, 0])), [0, 5, 0]);
    t.refuse = Some(3);
    let connect = [&[5u8, 1, 0, 3, 1][..], b"a", &80u16.to_be_bytes()].concat();
    let refused = front.serve(&mut t, PID, &on(2, 2, &connect));
    assert_eq!(refused[..3], [1, 5, 3]);
    assert_eq!(front.serve(&mut t, PID, &on(2, 2, &connect)), refused, "the kept answer");
}

#[test]
fn a_status_ask_says_how_far_and_touches_no_conversation() {
    let (mut front, mut t) = (Front::default(), fake());
    t.far = Far::Open;
    handshake(&mut front, &mut t);
    t.progress = Progress::of(1, false, false, false, 0);
    assert_eq!(front.serve(&mut t, PID, &[STATUS_ASK]), [STATUS, 1, 0, 2, 5]);
    assert!(t.closed.is_empty(), "the open stream is left alone");
}

#[test]
fn each_bootstrap_stage_is_its_own_step() {
    assert_eq!(Progress::of(0, false, false, false, 0).step, 1);
    assert_eq!(Progress::of(1, false, false, false, 0).step, 2);
    assert_eq!(Progress::of(2, false, false, false, 0).step, 3);
    assert_eq!(Progress::of(3, false, false, false, 0).step, 4, "no link to a guard yet");
    let building = Progress::of(3, false, true, true, 0);
    assert_eq!((building.step, building.ready), (5, false), "linked, no circuit yet");
    let ready = Progress::of(3, false, true, true, 1);
    assert_eq!((ready.step, ready.ready), (5, true));
    let refreshing = Progress::of(1, true, true, true, 2);
    assert!(refreshing.ready, "a consensus being refreshed still serves");
}
