// NONOS Operating System (AGPL-3.0-or-later)
//! A request runs until its answer: keep-alives say why it waits, the
//! answer is taken once, an error or a broken sequence ends it, and another
//! channel's reports never touch it.

use crate::ctaphid::Job;
use crate::keyside::{first, more};

const CID: [u8; 4] = [0xA, 0xB, 0xC, 0xD];

#[test]
fn keep_alives_say_the_key_waits_for_a_touch() {
    let mut j = Job::start(CID);
    assert_eq!(j.result(), Err((-11, Some(0))));
    assert!(j.hear(&first(CID, 0xBB, 1, &[2])));
    assert_eq!(j.result(), Err((-11, Some(2))), "user presence needed");
    assert!(!j.hear(&first([1; 4], 0x90, 1, &[0])), "another channel's");
    assert!(j.busy());
}

#[test]
fn an_answer_in_two_reports_is_taken_once() {
    let mut j = Job::start(CID);
    let answer: Vec<u8> = (0..80).collect();
    assert!(!j.hear(&first(CID, 0x90, 80, &answer[..57])));
    assert!(j.hear(&more(CID, 0, &answer[57..])));
    assert_eq!(j.result(), Ok(answer));
    assert_eq!(j.result(), Err((-22, None)), "once");
}

#[test]
fn an_error_or_a_broken_sequence_ends_it() {
    let mut j = Job::start(CID);
    assert!(j.hear(&first(CID, 0xBF, 1, &[0x06])));
    assert_eq!(j.result(), Err((-5, Some(0x06))), "channel busy");
    let mut j = Job::start(CID);
    j.hear(&first(CID, 0x90, 200, &[0; 57]));
    assert!(j.hear(&more(CID, 1, &[0; 59])), "sequence 1 before 0");
    assert_eq!(j.result(), Err((-5, None)));
    let mut j = Job::start(CID);
    assert!(j.hear(&first(CID, 0x90, 7610, &[0; 57])));
    assert_eq!(j.result(), Err((-5, None)), "longer than the framing carries");
}

#[test]
fn quiet_or_abandoned_requests_spend_the_channel() {
    let mut j = Job::start(CID);
    assert!(j.give_up());
    assert_eq!(j.result(), Err((-110, None)));
    assert!(!j.give_up(), "nothing ran");
    let mut j = Job::start(CID);
    assert!(j.abandon());
    assert!(!j.hear(&first(CID, 0x90, 1, &[0])), "its answer is no one's");
    let mut j = Job::start(CID);
    j.hear(&first(CID, 0x90, 1, &[0]));
    assert!(!j.abandon(), "answered: the channel is still good");
}
