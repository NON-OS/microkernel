// NONOS Operating System (AGPL-3.0-or-later)
/* Numbered exchanges, resets and frames the front does not speak. */

use super::socks::front::Front;
use super::socks::tunnel::Far;
use super::socks_fake::fake;
use super::socks_frames::{bytes, connect_example, handshake, numbered, HELLO, PID};

#[test]
fn a_lost_numbered_answer_is_given_again_without_taking_more() {
    let (mut front, mut t) = (Front::default(), fake());
    t.far = Far::Open;
    front.serve(&mut t, PID, &numbered(1, &HELLO));
    front.serve(&mut t, PID, &numbered(2, &connect_example()));
    t.arrived.extend_from_slice(b"first");
    let first = front.serve(&mut t, PID, &numbered(3, &[]));
    t.arrived.extend_from_slice(b"second");
    assert_eq!(front.serve(&mut t, PID, &numbered(3, &[])), first, "same number, same answer");
    assert_eq!(t.arrived, b"second", "the repeat took nothing off the stream");
    assert_eq!(&front.serve(&mut t, PID, &numbered(4, &[]))[..], b"\0second");
}

#[test]
fn the_answer_that_ends_a_conversation_can_be_asked_for_again() {
    let (mut front, mut t) = (Front::default(), fake());
    t.refuse = Some(3);
    front.serve(&mut t, PID, &numbered(1, &HELLO));
    let refused = front.serve(&mut t, PID, &numbered(2, &connect_example()));
    assert_eq!(refused[..3], [1, 5, 3]);
    assert_eq!(front.serve(&mut t, PID, &numbered(2, &connect_example())), refused);
}

#[test]
fn a_reset_ends_the_stream_and_starts_over() {
    let (mut front, mut t) = (Front::default(), fake());
    t.far = Far::Open;
    handshake(&mut front, &mut t);
    assert_eq!(front.serve(&mut t, PID, &[1]), [0]);
    assert_eq!(t.closed, [7]);
    assert_eq!(front.serve(&mut t, PID, &bytes(&HELLO)), [0, 5, 0], "a fresh greeting");
}

#[test]
fn a_frame_the_front_does_not_speak_closes_at_once() {
    let (mut front, mut t) = (Front::default(), fake());
    t.far = Far::Open;
    handshake(&mut front, &mut t);
    assert_eq!(front.serve(&mut t, PID, &[2, 1]), [1]);
    assert_eq!(t.closed, [7]);
}
