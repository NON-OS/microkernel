// NONOS Operating System (AGPL-3.0-or-later)
//! What a bound device answers in the tests after a bind: a command it
//! fails, and the check it passes after a request it failed.

use crate::fixture::sense;
use crate::xhci::Answer;

pub const OK: Answer = Answer::Csw { residue: 0, status: 0 };

/// A command the device fails, then `sense` as REQUEST SENSE reads it.
pub fn fails_with(sense: Vec<u8>) -> [Answer; 3] {
    [Answer::Csw { residue: 0, status: 1 }, Answer::Data(sense), OK]
}

/// A command the device fails with fixed-format sense `key` and `asc`.
pub fn fails(key: u8, asc: u8) -> [Answer; 3] {
    fails_with(sense(key, asc))
}

/// A check passed at `blocks` of 512 bytes: TEST UNIT READY, then READ
/// CAPACITY(10).
pub fn passes_check(blocks: u32) -> Vec<Answer> {
    let mut cap = (blocks - 1).to_be_bytes().to_vec();
    cap.extend_from_slice(&512u32.to_be_bytes());
    vec![OK, Answer::Data(cap), OK]
}
