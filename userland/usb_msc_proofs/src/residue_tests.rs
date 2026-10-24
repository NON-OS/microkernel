// NONOS Operating System (AGPL-3.0-or-later)
//! Residues that count nothing, found by a whole good INQUIRY or READ
//! CAPACITY that claims one (as observed, unverified), and true ones kept.

use crate::bot::CommandStatus;
use crate::disk::{capacity, inquiry, read, test_ready, Unit};
use crate::fixture::{class_resets, disk};
use crate::protocol::E_IO;
use crate::state::State;
use crate::xhci::{script, Answer};

const READ10: u8 = 0x28;

fn csw(residue: u32, status: u8) -> CommandStatus {
    CommandStatus { tag: 1, residue, status }
}

fn capacity_of_1000() -> Answer {
    let mut cap = 999u32.to_be_bytes().to_vec();
    cap.extend_from_slice(&512u32.to_be_bytes());
    Answer::Data(cap)
}

#[test]
fn a_stick_with_bogus_residues_is_found_by_its_inquiry_and_bound() {
    let mut state = State::new();
    let mut data = vec![0u8; 36];
    data[4] = 31;
    script(vec![
        Answer::Data(data),
        Answer::Csw { residue: 36, status: 0 },
        Answer::Csw { residue: 0, status: 0 },
    ]);
    assert!(matches!(inquiry(&disk(), &mut state), Ok(Some(_))), "named by its INQUIRY");
    assert_eq!(test_ready(&disk(), &mut state, &mut false), Ok(Unit::Ready));
    script(vec![capacity_of_1000(), Answer::Csw { residue: 8, status: 0 }]);
    assert_eq!(capacity(&disk(), &mut state), Ok((1000, 512)));
    script(vec![Answer::Data(vec![7; 512]), Answer::Csw { residue: 512, status: 0 }]);
    let mut buf = [0u8; 512];
    assert_eq!(read(&disk(), &mut state, 7, &mut buf), Ok(()), "its residue counts no more");
    assert_eq!(class_resets(), 0);
}

#[test]
fn a_stick_with_bogus_residues_is_found_by_its_read_capacity_as_well() {
    script(vec![capacity_of_1000(), Answer::Csw { residue: 8, status: 0 }]);
    assert_eq!(capacity(&disk(), &mut State::new()), Ok((1000, 512)));
}

#[test]
fn a_true_residue_is_kept() {
    // No INQUIRY or READ CAPACITY gave the device away: its residue on a
    // read that moved every byte says the data is not all good.
    script(vec![Answer::Data(vec![7; 512]), Answer::Csw { residue: 512, status: 0 }]);
    let mut buf = [0u8; 512];
    assert_eq!(read(&disk(), &mut State::new(), 7, &mut buf), Err(E_IO));
}

#[test]
fn the_residue_is_the_larger_of_the_hosts_and_the_csws() {
    let mut s = State::new();
    assert_eq!(s.residue(READ10, 512, 256, csw(0, 0)), 256, "the host saw half");
    assert_eq!(s.residue(READ10, 512, 512, csw(100, 0)), 100, "the device says");
    assert_eq!(s.residue(READ10, 512, 256, csw(100, 0)), 256);
    assert_eq!(s.residue(READ10, 512, 512, csw(9000, 0)), 512, "never past what was asked");
    assert_eq!(s.residue(0x00, 0, 0, csw(0, 0)), 0);
}
