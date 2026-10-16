// NONOS Operating System (AGPL-3.0-or-later)
//! UAS tags 1 and 2 by turns, so a Sense IU sent late for the command
//! before carries the other tag and is let go, never taken as the status
//! of the command in flight.

use crate::descriptors::MscBinding;
use crate::disk::{read, Disk};
use crate::state::State;
use crate::xhci::{device, script, Answer};

fn disk() -> Disk {
    let b = MscBinding { interface: 0, bulk_in: 0x85, bulk_out: 0x06, ..MscBinding::default() };
    let mut d = Disk::bound(1, 1, &b);
    (d.blocks, d.block_len, d.uas) = (1000, 512, true);
    d
}

fn iu(id: u8, tag: u8, status: u8) -> Answer {
    let mut iu = vec![id, 0, 0, tag];
    if id == 0x03 {
        iu.extend([0, 0, status, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }
    Answer::Data(iu)
}

#[test]
fn a_late_sense_for_the_command_before_is_let_go() {
    let (mut state, mut out) = (State::new(), [0u8; 512]);
    script(vec![iu(0x06, 1, 0), Answer::Data(vec![1; 512]), iu(0x03, 1, 0)]);
    assert_eq!(read(&disk(), &mut state, 0, &mut out), Ok(()));
    let late = iu(0x03, 1, 2);
    script(vec![late, iu(0x06, 2, 0), Answer::Data(vec![2; 512]), iu(0x03, 2, 0)]);
    assert_eq!(read(&disk(), &mut state, 0, &mut out), Ok(()), "not a CHECK CONDITION");
    assert_eq!(out, [2; 512]);
    let sent = device(|d| d.sent.clone());
    assert_eq!(sent[0].1[0..4], [0x01, 0, 0, 2], "the second command's tag");
}
