// NONOS Operating System (AGPL-3.0-or-later)
//! The BOT transport (`disk`) against a scripted device that does what real
//! sticks do and the Bulk-Only specification does not allow (as observed,
//! unverified): a zero-length packet ahead of the CSW, a CSW sent in place
//! of a data phase, and a data phase that ends short under a CSW that says
//! it did not (residue_tests holds residues that count nothing).

use crate::disk::read;
use crate::fixture::{answers_left, class_resets, disk, sense};
use crate::protocol::E_IO;
use crate::state::State;
use crate::xhci::{script, Answer};

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
        Answer::Data(sense(0x03, 0)),
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
