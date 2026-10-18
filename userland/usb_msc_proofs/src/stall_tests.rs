// NONOS Operating System (AGPL-3.0-or-later)
//! A stalled pipe and a CSW for another command. A stall is cleared on both
//! sides and the command carried on (BOT 1.0 section 6.6.1); a CSW whose tag
//! is not the command's is a lost phase, answered with the reset recovery
//! (section 5.3.4), and the next command runs clean.

use crate::descriptors::MscBinding;
use crate::disk::{read, Disk};
use crate::protocol::E_IO;
use crate::state::State;
use crate::xhci::{device, script, Answer};

const CLEAR_FEATURE: (u8, u8) = (0x02, 0x01);
const MASS_STORAGE_RESET: (u8, u8) = (0x21, 0xFF);

fn disk() -> Disk {
    let b = MscBinding { interface: 0, bulk_in: 0x81, bulk_out: 0x02, ..MscBinding::default() };
    let mut d = Disk::bound(1, 1, &b);
    (d.blocks, d.block_len) = (1000, 512);
    d
}

fn controls(req: (u8, u8)) -> Vec<u16> {
    device(|d| d.controls.iter().filter(|c| (c.0, c.1) == req).map(|c| c.3).collect())
}

fn sense(key: u8) -> Vec<u8> {
    let mut s = vec![0u8; 18];
    (s[0], s[2], s[7]) = (0x70, key, 10);
    s
}

#[test]
fn a_stalled_data_phase_is_cleared_and_its_csw_read() {
    script(vec![
        Answer::Stall,
        Answer::Csw { residue: 512, status: 1 },
        Answer::Data(sense(0x03)),
        Answer::Csw { residue: 0, status: 0 },
    ]);
    let mut buf = [0u8; 512];
    assert_eq!(read(&disk(), &mut State::new(), 7, &mut buf), Err(E_IO), "the read failed");
    assert_eq!(device(|d| d.resets.clone()), [true], "the IN pipe was reset in the controller");
    assert_eq!(controls(CLEAR_FEATURE), [0x81], "and its halt cleared in the device");
    assert!(controls(MASS_STORAGE_RESET).is_empty(), "the transport kept its phase");
    assert_eq!(device(|d| d.answers.len()), 0, "the sense was asked for");
}

#[test]
fn a_stalled_csw_is_read_again_once_the_halt_is_cleared() {
    script(vec![Answer::Data(vec![5; 512]), Answer::Stall, Answer::Csw { residue: 0, status: 0 }]);
    let mut buf = [0u8; 512];
    assert_eq!(read(&disk(), &mut State::new(), 7, &mut buf), Ok(()));
    assert_eq!(controls(CLEAR_FEATURE), [0x81]);
    assert!(controls(MASS_STORAGE_RESET).is_empty());
}

#[test]
fn a_csw_that_stalls_twice_is_a_lost_phase() {
    script(vec![Answer::Data(vec![5; 512]), Answer::Stall, Answer::Stall]);
    let mut buf = [0u8; 512];
    assert_eq!(read(&disk(), &mut State::new(), 7, &mut buf), Err(E_IO));
    assert_eq!(controls(MASS_STORAGE_RESET), [0], "the reset recovery, to interface 0");
}

#[test]
fn a_csw_for_another_tag_is_refused_and_the_device_reset() {
    let mut state = State::new();
    script(vec![Answer::Data(vec![5; 512]), Answer::CswTag { tag: 0xDEAD, residue: 0, status: 0 }]);
    let mut buf = [0u8; 512];
    assert_eq!(read(&disk(), &mut state, 7, &mut buf), Err(E_IO), "never taken as good");
    assert_eq!(controls(MASS_STORAGE_RESET), [0]);
    let pipes: Vec<bool> = device(|d| d.resets.clone());
    assert_eq!(pipes, [true, false], "both pipes cleared after the reset");
    script(vec![Answer::Data(vec![6; 512]), Answer::Csw { residue: 0, status: 0 }]);
    assert_eq!(read(&disk(), &mut state, 7, &mut buf), Ok(()), "the next command runs clean");
    assert!(buf.iter().all(|&b| b == 6));
}
