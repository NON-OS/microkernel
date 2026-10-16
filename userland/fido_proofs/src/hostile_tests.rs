// NONOS Operating System (AGPL-3.0-or-later)
//! A key that answers out of shape: a keep-alive with a status it has no
//! name for, an error or keep-alive of the wrong length, a sequence that
//! repeats, another channel's report mid-message, a message opened again
//! before the last ended, and a channel of zeroes from INIT.

use nonos_usbnet::mock::MockBus;

use crate::ctaphid::{allocate, Job};
use crate::keyside::{first, init_answer, more};

const CID: [u8; 4] = [0xA, 0xB, 0xC, 0xD];

#[test]
fn a_keep_alive_is_taken_only_with_a_known_status_and_one_byte() {
    let mut j = Job::start(CID);
    assert!(!j.hear(&first(CID, 0xBB, 1, &[3])), "status 3");
    assert!(!j.hear(&first(CID, 0xBB, 2, &[2, 0])), "two bytes");
    assert_eq!(j.result(), Err((-11, Some(0))));
    assert!(j.hear(&first(CID, 0xBB, 1, &[1])));
    assert_eq!(j.result(), Err((-11, Some(1))));
}

#[test]
fn an_error_of_another_length_ends_it_as_broken() {
    let mut j = Job::start(CID);
    assert!(j.hear(&first(CID, 0xBF, 2, &[0x06, 0])));
    assert_eq!(j.result(), Err((-5, None)));
}

#[test]
fn a_repeated_sequence_or_a_second_opening_breaks_the_message() {
    let mut j = Job::start(CID);
    j.hear(&first(CID, 0x90, 200, &[0; 57]));
    j.hear(&more(CID, 0, &[0; 59]));
    assert!(j.hear(&more(CID, 0, &[0; 59])), "sequence 0 again");
    assert_eq!(j.result(), Err((-5, None)));
    let mut j = Job::start(CID);
    j.hear(&first(CID, 0x90, 100, &[1; 57]));
    assert!(j.hear(&first(CID, 0x90, 1, &[2])), "opened again");
    assert_eq!(j.result(), Err((-5, None)));
}

#[test]
fn another_channel_mid_message_is_never_spliced_in() {
    let mut j = Job::start(CID);
    let answer: Vec<u8> = (0..80).collect();
    j.hear(&first(CID, 0x90, 80, &answer[..57]));
    assert!(!j.hear(&more([1; 4], 0, &[0xEE; 59])));
    assert!(j.hear(&more(CID, 0, &answer[57..])));
    assert_eq!(j.result(), Ok(answer));
}

#[test]
fn a_channel_of_zeroes_is_no_channel() {
    let mut b = MockBus::new(Box::new(|_| Ok(vec![])));
    let nonce = [3; 8];
    b.0.borrow_mut().notices.push_back(init_answer(nonce, [0; 4], 0x04));
    let mut looks = 0;
    let wait = || {
        looks += 1;
        looks <= 5
    };
    assert_eq!(allocate(&mut b, (3, 2), nonce, wait).map(|c| c.cid), Err(-110));
}
