// NONOS Operating System (AGPL-3.0-or-later)
//! Which answers give a device that reports bogus residues away, and that a
//! new device is not taken for the last one.

use crate::bot::CommandStatus;
use crate::state::State;

const READ10: u8 = 0x28;
const INQUIRY: u8 = 0x12;
const READ_CAPACITY10: u8 = 0x25;

fn csw(residue: u32, status: u8) -> CommandStatus {
    CommandStatus { tag: 1, residue, status }
}

#[test]
fn only_a_whole_good_inquiry_or_read_capacity_gives_bogus_residues_away() {
    let mut s = State::new();
    assert_eq!(s.residue(INQUIRY, 36, 36, csw(5, 1)), 5, "a failed INQUIRY");
    assert_eq!(s.residue(INQUIRY, 36, 30, csw(6, 0)), 6, "a short INQUIRY");
    assert_eq!(s.residue(INQUIRY, 96, 96, csw(5, 0)), 5, "not the 36 bytes Linux asks");
    assert_eq!(s.residue(READ10, 512, 512, csw(5, 0)), 5, "not a probe command");
    assert_eq!(s.residue(READ_CAPACITY10, 8, 8, csw(8, 0)), 0, "given away");
    assert_eq!(s.residue(READ10, 512, 512, csw(512, 0)), 0, "ignored from then on");
    assert_eq!(s.residue(READ10, 512, 256, csw(512, 0)), 256, "what the host saw still counts");
}

#[test]
fn a_new_device_is_not_taken_for_the_last_one() {
    let mut s = State::new();
    assert_eq!(s.residue(INQUIRY, 36, 36, csw(36, 0)), 0);
    s.install_bindings(&crate::descriptors::ProbeResult::empty());
    assert_eq!(s.residue(READ10, 512, 512, csw(512, 0)), 512);
}
