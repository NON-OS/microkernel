// NONOS Operating System (AGPL-3.0-or-later)
//! SERIAL_STATE read from the notification endpoint, and the state a
//! client reads: lines as they stand, events until read.

use crate::acm::{serial_state, State};

fn notice(kind: u8, length: u16, data: &[u8]) -> Vec<u8> {
    let l = length.to_le_bytes();
    let mut n = vec![0xA1, kind, 0, 0, 0, 0, l[0], l[1]];
    n.extend(data);
    n
}

#[test]
fn serial_state_is_read_and_others_are_not() {
    assert_eq!(serial_state(&notice(0x20, 2, &[0x03, 0x00])), Some(0x03));
    assert_eq!(serial_state(&notice(0x20, 2, &[0xFF, 0xFF])), Some(0x7F), "reserved bits");
    assert_eq!(serial_state(&notice(0x00, 0, &[])), None, "NETWORK_CONNECTION");
    assert_eq!(serial_state(&notice(0x20, 2, &[0x03])), None, "short");
    let mut wrong = notice(0x20, 2, &[1, 0]);
    wrong[0] = 0x21;
    assert_eq!(serial_state(&wrong), None, "not from an interface");
}

#[test]
fn levels_follow_the_line_and_events_wait_to_be_read() {
    let mut s = State::default();
    s.seen(0x01 | 0x40); // carrier, and an overrun
    s.seen(0x02); // carrier lost, DSR up
    assert_eq!(s.take(), 0x02 | 0x40);
    assert_eq!(s.take(), 0x02, "the overrun was read");
}
