// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Each way a proof does not come, in the note's words (section 4), and
//! "Run it again" under every one past Reading, where the run is used up.

use super::proof_fail::Fail;
use super::proof_said::{again, said, AGAIN};
use super::proof_seq::{Phase, DEVICE};
use super::prove_seq::walk;

const WORDS: [(Fail, &str); 14] = [
    (Fail::NoBox, "The market has no box for this listing to prove against"),
    (Fail::NotNamed, "The market's answer does not name this listing; not asked"),
    (Fail::MayNot, "This window may not ask for a proof"),
    (Fail::Busy, "The prover is busy: close its window, or try again in a few minutes"),
    (Fail::NotStarted, "The prover did not start; the serial log says why"),
    (Fail::Code(-2), "No finished run to prove: run age, close it, then press p"),
    (Fail::Code(-1101), "This machine's registry copy is out of date"),
    (Fail::Code(-1102), "This run is not of this listing's box"),
    (Fail::Code(-1103), "This device is not enrolled"),
    (Fail::Code(-1104), "The proof did not come out right"),
    (Fail::Code(-1105), "A call outside its box succeeded: there is nothing true to prove"),
    (Fail::Code(-12), "Not enough memory for the prover"),
    (Fail::Quiet, "The prover stopped answering"),
    (Fail::Broken, "The proof came back broken"),
];

#[test]
fn every_failure_has_its_own_words() {
    for (f, want) in WORDS {
        assert_eq!(said(f, b"age", true), want.as_bytes(), "{f:?}");
    }
    assert_eq!(said(Fail::Code(-3), b"age", true), b"The prover stopped; the serial log says why");
    let unknown = said(Fail::Code(-1102), b"age", false);
    let b = "The market's box for age changed since it was installed; reinstall it to prove it";
    assert_eq!(unknown, b.as_bytes(), "proved against the default release");
}

#[test]
fn a_failure_past_reading_says_run_it_again() {
    assert_eq!(again(true), Some(AGAIN));
    assert_eq!(AGAIN, b"Run it again to prove it again");
    assert_eq!(again(false), None);
    let proving = Phase::Proving { stage: DEVICE, out: true };
    let past = |e| match walk(proving, &[e]).0 {
        Phase::Failed(_, past) => again(past),
        p => panic!("{p:?}"),
    };
    use super::proof_event::Event;
    assert_eq!(past(Event::Status(Err(-1103))), Some(AGAIN));
    assert_eq!(past(Event::Late), Some(AGAIN));
    assert_eq!(past(Event::Status(Err(-2))), None, "no sealed run was read");
    assert_eq!(Fail::Code(-1103).code(), b"-1103");
    assert_eq!((Fail::Busy.code(), Fail::Broken.code()), (b"-16".to_vec(), b"broken".to_vec()));
}
