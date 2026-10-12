// NONOS Operating System (AGPL-3.0-or-later)
//! Commands built and answers read: the header's fields where the class
//! puts them, and an answer whose dwLength overruns what came refused.

use crate::ccid::{answers, command, reply, Outcome, GET_SLOT_STATUS, ICC_POWER_ON, XFR_BLOCK};
use crate::reader::answer;

#[test]
fn a_command_is_its_header_then_its_data() {
    let m = command(XFR_BLOCK, 1, 9, &[0x00, 0xA4, 0x04, 0x00]);
    assert_eq!(m, [0x6F, 4, 0, 0, 0, 1, 9, 0, 0, 0, 0x00, 0xA4, 0x04, 0x00]);
    assert_eq!(command(ICC_POWER_ON, 0, 3, &[]), [0x62, 0, 0, 0, 0, 0, 3, 0, 0, 0]);
}

#[test]
fn an_answer_reads_its_outcome_and_card_state() {
    let ok = answer(0x80, 0, 5, 0x00, 0, &[0x90, 0x00]);
    let r = reply(&ok).unwrap();
    assert_eq!((r.kind, r.slot, r.seq, r.icc, r.outcome), (0x80, 0, 5, 0, Outcome::Done));
    assert_eq!(r.data, [0x90, 0x00]);
    let gone = answer(0x81, 0, 6, 0x42, 0xFE, &[]);
    let gone = reply(&gone).unwrap();
    assert_eq!((gone.icc, gone.outcome), (2, Outcome::Failed(0xFE)));
    let wait = answer(0x80, 0, 7, 0x80, 1, &[]);
    assert_eq!(reply(&wait).unwrap().outcome, Outcome::MoreTime);
    assert_eq!(reply(&answer(0x80, 0, 7, 0xC0, 0, &[])), None, "status 3 is no outcome");
}

#[test]
fn an_answer_shorter_than_its_length_is_none() {
    let mut a = answer(0x80, 0, 5, 0, 0, &[1, 2, 3]);
    a.pop();
    assert_eq!(reply(&a), None);
    assert_eq!(reply(&a[..9]), None);
}

#[test]
fn each_command_has_the_answer_kind_it_expects() {
    assert_eq!(answers(ICC_POWER_ON), 0x80);
    assert_eq!(answers(XFR_BLOCK), 0x80);
    assert_eq!(answers(GET_SLOT_STATUS), 0x81);
    assert_eq!(answers(0x63), 0x81);
}
