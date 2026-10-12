// NONOS Operating System (AGPL-3.0-or-later)
//! Line coding's bytes, what is a line, and the class requests' setups.

use crate::acm::{control_lines, send_break, set_line, Line, DEFAULT, DTR, RTS};
use nonos_usbnet::Setup;

#[test]
fn line_coding_is_seven_bytes_rate_first() {
    assert_eq!(DEFAULT.bytes(), [0x00, 0xC2, 0x01, 0x00, 0, 0, 8]);
    assert_eq!(Line::from_bytes(&DEFAULT.bytes()), Some(DEFAULT));
    let odd = Line { baud: 9600, stop: 2, parity: 1, data: 7 };
    assert_eq!(Line::from_bytes(&odd.bytes()), Some(odd));
}

#[test]
fn what_is_not_a_line_is_refused() {
    let bad = |l: Line| Line::from_bytes(&l.bytes());
    assert_eq!(bad(Line { baud: 0, ..DEFAULT }), None);
    assert_eq!(bad(Line { stop: 3, ..DEFAULT }), None);
    assert_eq!(bad(Line { parity: 5, ..DEFAULT }), None);
    assert_eq!(bad(Line { data: 9, ..DEFAULT }), None);
    assert_eq!(bad(Line { data: 16, ..DEFAULT }).map(|l| l.data), Some(16));
    assert_eq!(Line::from_bytes(&[0; 6]), None, "short");
}

#[test]
fn the_requests_go_to_the_communications_interface() {
    assert_eq!(set_line(2), Setup::new(0x21, 0x20, 0, 2));
    assert_eq!(control_lines(2, DTR | RTS), Setup::new(0x21, 0x22, 3, 2));
    assert_eq!(control_lines(2, 0xFFFF), Setup::new(0x21, 0x22, 3, 2), "only DTR and RTS");
    assert_eq!(send_break(2, 250), Setup::new(0x21, 0x23, 250, 2));
}
