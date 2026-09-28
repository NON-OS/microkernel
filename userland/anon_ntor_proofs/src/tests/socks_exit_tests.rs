// NONOS Operating System (AGPL-3.0-or-later)
/* What the exit does to a stream, and what the caller is handed for it. */

use super::socks::front::Front;
use super::socks::stage::OUT_MAX;
use super::socks::tunnel::Far;
use super::socks_fake::fake;
use super::socks_frames::{bytes, handshake, PID};

#[test]
fn an_exit_refusal_maps_its_end_reason_and_frees_the_stream() {
    for (reason, rep) in [(2u8, 4u8), (3, 5), (4, 2), (7, 6), (8, 3), (1, 1), (6, 1)] {
        let (mut front, mut t) = (Front::default(), fake());
        handshake(&mut front, &mut t);
        t.far = Far::Ended(reason);
        assert_eq!(front.serve(&mut t, PID, &bytes(&[]))[..3], [1, 5, rep], "END {reason}");
        assert_eq!(t.closed, [7]);
    }
}

#[test]
fn bytes_cross_both_ways_once_connected() {
    let (mut front, mut t) = (Front::default(), fake());
    t.far = Far::Open;
    assert_eq!(handshake(&mut front, &mut t)[..3], [0, 5, 0]);
    t.arrived.extend_from_slice(b"server hello");
    let answer = front.serve(&mut t, PID, &bytes(b"client hello"));
    assert_eq!(t.sent, b"client hello");
    assert_eq!(&answer[..], b"\0server hello");
}

#[test]
fn an_ended_stream_hands_back_everything_before_closing() {
    let (mut front, mut t) = (Front::default(), fake());
    t.far = Far::Open;
    handshake(&mut front, &mut t);
    t.arrived = alloc::vec![0x5A; OUT_MAX + 100];
    t.far = Far::Ended(6);
    let first = front.serve(&mut t, PID, &bytes(&[]));
    assert_eq!((first[0], first.len()), (0, 1 + OUT_MAX), "still open while bytes wait");
    let last = front.serve(&mut t, PID, &bytes(&[]));
    assert_eq!((last[0], last.len()), (1, 1 + 100));
    assert_eq!(t.closed, [7]);
}

#[test]
fn bytes_held_for_a_window_go_on_the_next_poll() {
    let (mut front, mut t) = (Front::default(), fake());
    t.far = Far::Open;
    handshake(&mut front, &mut t);
    t.blocked = true;
    assert_eq!(front.serve(&mut t, PID, &bytes(b"request")), [0]);
    assert!(t.sent.is_empty());
    t.blocked = false;
    front.serve(&mut t, PID, &bytes(&[]));
    assert_eq!(t.sent, b"request");
}
