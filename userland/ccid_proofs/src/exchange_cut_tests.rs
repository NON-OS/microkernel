// NONOS Operating System (AGPL-3.0-or-later)
//! Each bulk IN transfer is one message. An answer cut short of its
//! dwLength is passed over, never joined to what the next transfer brings:
//! joined, our header would take another message's bytes as its data.

use nonos_usbnet::mock::Call;

use crate::ccid::{command, exchange, XFR_BLOCK};
use crate::exchange_tests::{reader, APDU};
use crate::reader::answer;

#[test]
fn an_answer_cut_short_is_passed_over_for_the_whole_one() {
    let cut = answer(0x80, 0, 4, 0, 0, &[0x5A; 70])[..64].to_vec();
    let whole = answer(0x80, 0, 4, 0, 0, &[0x90, 0x00]);
    let mut bus = reader(vec![Ok(Some(cut)), Ok(None), Ok(Some(whole.clone()))]);
    // Bounded, so a read that joins them fails here rather than waits on.
    let mut looks = 0;
    let wait = |_| {
        looks += 1;
        looks <= 10
    };
    assert_eq!(exchange(&mut bus, &command(XFR_BLOCK, 0, 4, &APDU), 271, wait), Ok(whole));
    assert_eq!(bus.0.borrow().calls, [Call::BulkOut(command(XFR_BLOCK, 0, 4, &APDU))]);
}

#[test]
fn a_cut_answer_never_takes_the_next_message_as_its_data() {
    // Our header claims 70 bytes; the next transfer is another message of
    // 60, so header and all it would make up the 70.
    let ours = answer(0x80, 0, 4, 0, 0, &[0x5A; 70])[..20].to_vec();
    let next = answer(0x80, 0, 9, 0, 0, &[0x11; 50]);
    let mut bus = reader(vec![Ok(Some(ours)), Ok(Some(next))]);
    let mut looks = 0;
    let wait = |_| {
        looks += 1;
        looks <= 5
    };
    assert_eq!(exchange(&mut bus, &command(XFR_BLOCK, 0, 4, &APDU), 271, wait), Err(-110));
}
