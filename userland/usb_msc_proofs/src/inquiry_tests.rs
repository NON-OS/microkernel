// NONOS Operating System (AGPL-3.0-or-later)
//! An INQUIRY answered with CHECK CONDITION: its sense is taken, which
//! clears it, and the INQUIRY is sent once more. Only a good INQUIRY names
//! the unit (scan/identity.rs). The unfixed driver took the failed one as
//! asked, with its condition left pending, and named the unit by nothing.

use crate::disk::inquiry;
use crate::fixture::{disk, sense};
use crate::state::State;
use crate::xhci::{device, script, Answer};

const OK: Answer = Answer::Csw { residue: 0, status: 0 };
const REQUEST_SENSE: u8 = 0x03;
const INQUIRY: u8 = 0x12;

/// An INQUIRY the unit fails with its CSW in place of the data, and the
/// sense taken after it.
fn failed() -> [Answer; 3] {
    [Answer::Csw { residue: 36, status: 1 }, Answer::Data(sense(0x06, 0x29)), OK]
}

fn named(answers: Vec<Answer>) -> (Result<Option<u64>, i32>, Vec<u8>) {
    script(answers);
    let named = inquiry(&disk(), &mut State::new());
    (named, device(|d| d.commands.clone()))
}

#[test]
fn an_inquiry_failed_once_is_asked_again_after_its_sense() {
    let good = || [Answer::Data(b"NONOS stick".repeat(4)), OK];
    let (clean, _) = named(good().into());
    let (again, sent) = named(failed().into_iter().chain(good()).collect());
    assert!(matches!(clean, Ok(Some(_))));
    assert_eq!(again, clean, "named by the good INQUIRY");
    assert_eq!(sent, vec![INQUIRY, REQUEST_SENSE, INQUIRY]);
}

#[test]
fn an_inquiry_failed_twice_names_nothing() {
    let (named, sent) = named(failed().into_iter().chain(failed()).collect());
    assert_eq!(named, Ok(None));
    assert_eq!(sent, vec![INQUIRY, REQUEST_SENSE, INQUIRY, REQUEST_SENSE]);
}
