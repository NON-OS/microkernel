// NONOS Operating System (AGPL-3.0-or-later)
//! A port the HBA reset spares, as on a part that resets only its global
//! state: whichever engine bits it was left showing, the driver stops it on
//! its own (ST off and CR waited, then FRE off and FR waited, AHCI 1.3.1,
//! 10.1.2), and the controller is kept, Bus Master on only after the stop.

use nonos_libc::{bus_master_after_reset, seen};

use super::open::open;
use super::open_tests::AHCI;
use crate::constants::regs::{CMD_CR, CMD_FR, CMD_FRE, CMD_ST};
use crate::model::HbaRig;

/// Open an HBA whose reset leaves port 0's PxCMD at `cmd`.
fn spared(cmd: u32) {
    let rig = HbaRig::left_armed();
    rig.hba().quirks.hr_spares_port = true;
    rig.hba().reg.cmd = cmd;
    let opened = open(AHCI).expect("the port stops on a stop of its own");
    bus_master_after_reset(&seen());
    let said = nonos_libc::said();
    assert!(said.iter().any(|l| l.contains("port 0 still armed after the HBA reset")), "{said:?}");
    drop(opened);
}

#[test]
fn a_port_the_reset_spares_is_stopped_and_the_controller_kept() {
    spared(CMD_ST | CMD_CR | CMD_FRE | CMD_FR);
}

#[test]
fn a_port_spared_with_fis_receive_alone_on_is_stopped_too() {
    spared(CMD_FRE | CMD_FR);
}
