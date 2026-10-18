// NONOS Operating System (AGPL-3.0-or-later)
//! A halted pipe, either way, is cleared and the command failed; an answer
//! longer than a message may be is refused.

use nonos_usbnet::mock::Call;

use crate::ccid::{command, exchange, XFR_BLOCK};
use crate::exchange_tests::{reader, APDU};
use crate::reader::answer;

#[test]
fn a_halted_pipe_is_cleared_and_an_overlong_answer_refused() {
    let mut bus = reader(vec![Err(-32)]);
    assert_eq!(exchange(&mut bus, &command(XFR_BLOCK, 0, 4, &APDU), 271, |_| true), Err(-5));
    assert_eq!(bus.0.borrow().calls.last(), Some(&Call::ResetBulk(true)));
    let mut bus = reader(vec![]);
    bus.0.borrow_mut().bulk_out_fails = Some(-32);
    assert_eq!(exchange(&mut bus, &command(XFR_BLOCK, 0, 4, &APDU), 271, |_| true), Err(-5));
    assert_eq!(bus.0.borrow().calls.last(), Some(&Call::ResetBulk(false)));
    let mut bus = reader(vec![Ok(Some(answer(0x80, 0, 4, 0, 0, &[0; 262])))]);
    assert_eq!(exchange(&mut bus, &command(XFR_BLOCK, 0, 4, &APDU), 271, |_| true), Err(-5));
}
