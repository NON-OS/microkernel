// NONOS Operating System (AGPL-3.0-or-later)
/* The bounds: unsent bytes per caller, and callers at once. */

use super::socks::front::{Front, CALLERS_MAX};
use super::socks::stage::UNSENT_MAX;
use super::socks::tunnel::Far;
use super::socks_fake::fake;
use super::socks_frames::{bytes, handshake, HELLO, PID};

#[test]
fn a_caller_writing_past_the_limit_is_closed() {
    let (mut front, mut t) = (Front::default(), fake());
    t.far = Far::Open;
    handshake(&mut front, &mut t);
    t.blocked = true;
    let chunk = alloc::vec![1u8; UNSENT_MAX / 4];
    for _ in 0..4 {
        assert_eq!(front.serve(&mut t, PID, &bytes(&chunk)), [0]);
    }
    assert_eq!(front.serve(&mut t, PID, &bytes(&[1])), [1]);
    assert_eq!(t.closed, [7]);
}

#[test]
fn a_full_table_refuses_the_next_caller_and_keeps_the_rest() {
    let (mut front, mut t) = (Front::default(), fake());
    for pid in 0..CALLERS_MAX as u32 {
        assert_eq!(front.serve(&mut t, pid, &bytes(&[5])), [0]);
    }
    assert_eq!(front.serve(&mut t, 999, &bytes(&HELLO)), [1, 5, 0xFF]);
    assert_eq!(front.serve(&mut t, 0, &bytes(&[1, 0])), [0, 5, 0], "an existing caller carries on");
}

#[test]
fn a_stream_gone_from_the_table_ends_the_conversation() {
    let (mut front, mut t) = (Front::default(), fake());
    handshake(&mut front, &mut t);
    t.far = Far::Gone;
    assert_eq!(front.serve(&mut t, PID, &bytes(&[])), [1, 5, 1, 0, 1, 0, 0, 0, 0, 0, 0]);
}
