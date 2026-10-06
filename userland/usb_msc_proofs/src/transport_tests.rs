// NONOS Operating System (AGPL-3.0-or-later)
//! The BOT transport (`disk`) against a scripted device that does what real
//! sticks do and the Bulk-Only specification does not allow, as Linux's
//! usb_stor_Bulk_transport copes with it: a zero-length packet ahead of the
//! CSW, a CSW sent in place of a data phase, a data phase that ends short
//! under a CSW that says it did not, and residues that count nothing.

use crate::bot::CommandStatus;
use crate::descriptors::MscBinding;
use crate::disk::{capacity, read, unit_ready, Disk};
use crate::protocol::E_IO;
use crate::state::State;
use crate::xhci::{device, script, Answer};

const READ10: u8 = 0x28;
const INQUIRY: u8 = 0x12;
const READ_CAPACITY10: u8 = 0x25;

fn disk() -> Disk {
    let b = MscBinding { interface: 0, bulk_in: 0x81, bulk_out: 0x02, ..MscBinding::default() };
    let mut d = Disk::bound(1, 1, &b);
    (d.blocks, d.block_len) = (1000, 512);
    d
}

/// Bulk-Only Mass Storage Resets the transport sent.
fn class_resets() -> usize {
    device(|d| d.controls.iter().filter(|c| (c.0, c.1) == (0x21, 0xFF)).count())
}

fn answers_left() -> usize {
    device(|d| d.answers.len())
}

/// Fixed-format sense data with `key`.
fn sense(key: u8) -> Vec<u8> {
    let mut s = vec![0u8; 18];
    (s[0], s[2], s[7]) = (0x70, key, 10);
    s
}

fn csw(residue: u32, status: u8) -> CommandStatus {
    CommandStatus { tag: 1, residue, status }
}

#[test]
fn a_zero_length_packet_ahead_of_the_csw_is_read_past() {
    script(vec![Answer::Data(vec![0xAB; 512]), Answer::Zlp, Answer::Csw { residue: 0, status: 0 }]);
    let mut buf = [0u8; 512];
    assert_eq!(read(&disk(), &mut State::new(), 7, &mut buf), Ok(()));
    assert!(buf.iter().all(|&b| b == 0xAB));
    assert_eq!(class_resets(), 0, "the transport was never reset");
    assert_eq!(answers_left(), 0);
}

#[test]
fn a_csw_sent_in_place_of_the_data_is_taken_as_the_csw() {
    // A READ the device fails without its data phase: the CSW comes as the
    // first 13 bytes, and the sense is asked for at once, with no wait for a
    // second CSW that never comes and no reset.
    script(vec![
        Answer::Csw { residue: 512, status: 1 },
        Answer::Data(sense(0x03)),
        Answer::Csw { residue: 0, status: 0 },
    ]);
    let mut buf = [0u8; 512];
    assert_eq!(read(&disk(), &mut State::new(), 7, &mut buf), Err(E_IO));
    assert_eq!(class_resets(), 0, "the transport was never reset");
    assert_eq!(answers_left(), 0, "REQUEST SENSE was asked and answered");
}

#[test]
fn thirteen_short_bytes_without_the_csw_signature_are_data() {
    let mut short = vec![0u8; 13];
    short[0..4].copy_from_slice(b"USBC");
    script(vec![Answer::Data(short), Answer::Csw { residue: 499, status: 0 }]);
    let mut state = State::new();
    let mut buf = [0u8; 512];
    assert_eq!(read(&disk(), &mut state, 7, &mut buf), Err(E_IO), "a short read is not whole");
    assert_eq!(class_resets(), 0, "the CSW was read where it belongs");
    assert_eq!(answers_left(), 0);
}

#[test]
fn a_short_data_phase_under_a_csw_of_no_residue_is_not_whole() {
    // Half a block came in. The CSW says all of it did; the host knows
    // better, and the half block never received is not handed on as data.
    script(vec![Answer::Data(vec![1; 256]), Answer::Csw { residue: 0, status: 0 }]);
    let mut buf = [0u8; 512];
    assert_eq!(read(&disk(), &mut State::new(), 7, &mut buf), Err(E_IO));
    assert_eq!(answers_left(), 0);
}

#[test]
fn a_stick_with_bogus_residues_is_found_by_its_inquiry_and_bound() {
    let mut state = State::new();
    let mut inquiry = vec![0u8; 36];
    inquiry[4] = 31;
    script(vec![
        Answer::Data(inquiry),
        Answer::Csw { residue: 36, status: 0 },
        Answer::Csw { residue: 0, status: 0 },
    ]);
    assert_eq!(unit_ready(&disk(), &mut state, false), Ok(()));
    let mut cap = 999u32.to_be_bytes().to_vec();
    cap.extend_from_slice(&512u32.to_be_bytes());
    script(vec![Answer::Data(cap), Answer::Csw { residue: 8, status: 0 }]);
    assert_eq!(capacity(&disk(), &mut state), Ok((1000, 512)));
    script(vec![Answer::Data(vec![7; 512]), Answer::Csw { residue: 512, status: 0 }]);
    let mut buf = [0u8; 512];
    assert_eq!(read(&disk(), &mut state, 7, &mut buf), Ok(()), "its residue counts no more");
    assert_eq!(class_resets(), 0);
}

#[test]
fn a_stick_with_bogus_residues_is_found_by_its_read_capacity_as_well() {
    let mut cap = 999u32.to_be_bytes().to_vec();
    cap.extend_from_slice(&512u32.to_be_bytes());
    script(vec![Answer::Data(cap), Answer::Csw { residue: 8, status: 0 }]);
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
