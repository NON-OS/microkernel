// NONOS Operating System (AGPL-3.0-or-later)
//! Readiness: an empty slot given up at once, and the caller's pause, where
//! the server answers its inbox, run between tries.

use crate::descriptors::MscBinding;
use crate::disk::{unit_ready, Disk};
use crate::protocol::{E_IO, E_NOMEDIUM};
use crate::state::State;
use crate::xhci::{device, script, Answer};

fn disk() -> Disk {
    let b = MscBinding { interface: 0, bulk_in: 0x81, bulk_out: 0x02, ..MscBinding::default() };
    Disk::bound(1, 1, &b)
}

fn sense(key: u8, asc: u8) -> Vec<u8> {
    let mut s = vec![0u8; 18];
    (s[0], s[2], s[7], s[12]) = (0x70, key, 10, asc);
    s
}

fn inquiry_answered() -> Vec<Answer> {
    vec![Answer::Data(vec![0; 36]), Answer::Csw { residue: 0, status: 0 }]
}

#[test]
fn a_slot_with_no_medium_is_given_up_at_once() {
    let mut script_ = inquiry_answered();
    script_.push(Answer::Csw { residue: 0, status: 1 });
    script_.extend([Answer::Data(sense(0x02, 0x3A)), Answer::Csw { residue: 0, status: 0 }]);
    script(script_);
    let mut pauses = 0;
    let got = unit_ready(&disk(), &mut State::new(), &mut |_| pauses += 1);
    assert_eq!(got, Err(E_NOMEDIUM));
    assert_eq!(pauses, 0, "no wait for a card that is not there");
    assert_eq!(device(|d| d.answers.len()), 0);
}

#[test]
fn a_medium_spinning_up_is_started_once_and_the_pause_runs_between_tries() {
    let mut script_ = inquiry_answered();
    script_.push(Answer::Csw { residue: 0, status: 1 });
    script_.extend([Answer::Data(sense(0x02, 0x04)), Answer::Csw { residue: 0, status: 0 }]);
    script_.push(Answer::Csw { residue: 0, status: 0 });
    script_.push(Answer::Csw { residue: 0, status: 0 });
    script(script_);
    let mut pauses = 0;
    assert_eq!(unit_ready(&disk(), &mut State::new(), &mut |_| pauses += 1), Ok(()));
    assert_eq!(pauses, 1, "one pause, between the two TEST UNIT READYs");
    assert_eq!(device(|d| d.answers.len()), 0, "START UNIT was sent once");
}

#[test]
fn an_inquiry_that_never_completes_ends_after_its_tries_with_a_pause_between_each() {
    script(Vec::new());
    let mut pauses = 0;
    assert_eq!(unit_ready(&disk(), &mut State::new(), &mut |_| pauses += 1), Err(E_IO));
    assert_eq!(pauses, 4, "five tries, four pauses");
}
