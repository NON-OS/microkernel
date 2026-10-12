// NONOS Operating System (AGPL-3.0-or-later)
//! One command and its answer through a scripted reader: the answer
//! gathered across transfers, a late answer let go, time extensions taken
//! up to their bound, and the wait ended when time runs out.

use nonos_usbnet::mock::{Call, MockBus};

use crate::ccid::{command, exchange, XFR_BLOCK};
use crate::reader::answer;

pub(crate) fn reader(transfers: Vec<Result<Option<Vec<u8>>, i32>>) -> MockBus {
    let bus = MockBus::new(Box::new(|_| Ok(vec![])));
    bus.0.borrow_mut().bulk_in.extend(transfers);
    bus
}

pub(crate) const APDU: [u8; 5] = [0x00, 0xCA, 0x00, 0x6E, 0x00];

#[test]
fn an_answer_in_two_transfers_is_read_whole() {
    let a = answer(0x80, 0, 4, 0, 0, &[0x5A; 70]);
    let mut bus = reader(vec![Ok(Some(a[..64].to_vec())), Ok(None), Ok(Some(a[64..].to_vec()))]);
    let got = exchange(&mut bus, &command(XFR_BLOCK, 0, 4, &APDU), 271, |_| true);
    assert_eq!(got, Ok(a));
    assert_eq!(bus.0.borrow().calls, [Call::BulkOut(command(XFR_BLOCK, 0, 4, &APDU))]);
}

#[test]
fn a_late_answer_to_an_earlier_command_is_let_go() {
    let late = answer(0x80, 0, 3, 0, 0, &[0x6A, 0x82]);
    let ours = answer(0x80, 0, 4, 0, 0, &[0x90, 0x00]);
    let mut bus = reader(vec![Ok(Some(late)), Ok(Some(ours.clone()))]);
    let got = exchange(&mut bus, &command(XFR_BLOCK, 0, 4, &APDU), 271, |_| true);
    assert_eq!(got, Ok(ours));
}

#[test]
fn time_extensions_start_the_wait_again_up_to_their_bound() {
    let more = || Ok(Some(answer(0x80, 0, 4, 0x80, 1, &[])));
    let done = answer(0x80, 0, 4, 0, 0, &[0x90, 0x00]);
    let mut bus = reader(vec![more(), more(), Ok(Some(done.clone()))]);
    let mut fresh = 0;
    let wait = |f: bool| {
        fresh += f as u32;
        true
    };
    assert_eq!(exchange(&mut bus, &command(XFR_BLOCK, 0, 4, &APDU), 271, wait), Ok(done));
    assert_eq!(fresh, 3, "the first wait and one after each extension");
    // EXTENSIONS is 64: the 65th is one too many.
    let mut bus = reader((0..65).map(|_| more()).collect());
    let got = exchange(&mut bus, &command(XFR_BLOCK, 0, 4, &APDU), 271, |_| true);
    assert_eq!(got, Err(-110));
}

#[test]
fn a_reader_that_never_answers_runs_out_of_time() {
    let mut bus = reader(vec![]);
    let mut looks = 0;
    let wait = |_| {
        looks += 1;
        looks <= 10
    };
    assert_eq!(exchange(&mut bus, &command(XFR_BLOCK, 0, 4, &APDU), 271, wait), Err(-110));
    assert_eq!(looks, 11);
}
