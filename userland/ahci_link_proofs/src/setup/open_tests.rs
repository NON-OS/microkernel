// NONOS Operating System (AGPL-3.0-or-later)
//! Bus Master at open, on an HBA a driver that died without its Drop left
//! with port 0's engines armed on frames the kernel has since freed: it goes
//! on only once the HBA reset is seen to end and every port reads stopped
//! (a port the reset spared stopped on its own, `spared_tests`), never for a
//! controller refused, and the release is the last call, no DMA grant
//! unmapped by hand before it.

use nonos_libc::{bus_master_after_reset, refuse, refused_without_bus_master, released_last};
use nonos_libc::{seen, Call};

use super::open::open;
use crate::constants::regs::{GHC_AE, HBA_GHC};
use crate::discover::Found;
use crate::error::AhciError;
use crate::model::{CrStuck, HbaRig};

pub(super) const AHCI: Found =
    Found { device_id: 9, vendor: 0x8086, irq_line: 11, abar_size: 0x2000 };

#[test]
fn an_hba_left_armed_is_reset_before_bus_master_goes_on() {
    let _rig = HbaRig::left_armed();
    let opened = open(AHCI).expect("the controller opens");
    bus_master_after_reset(&seen());
    // SAFETY: the model HBA is plugged in; no host memory is reached.
    let ghc = unsafe { opened.regs.r32(HBA_GHC) };
    let port = opened.ports[0];
    assert_eq!((opened.epoch, opened.info.pi, port.implemented, ghc & GHC_AE), (3, 1, 1, GHC_AE));
    assert_eq!(opened.handles.irq_grant_id(), 0x41, "the interrupt the broker bound");
    drop(opened);
    released_last(&seen());
}

#[test]
fn an_hba_whose_reset_never_ends_is_refused_without_bus_master() {
    let rig = HbaRig::left_armed();
    rig.hba().global.as_mut().expect("a whole HBA").hr_stuck = true;
    assert_eq!(open(AHCI).err(), Some(AhciError::Timeout));
    refused_without_bus_master(&seen());
}

#[test]
fn an_abar_the_broker_will_not_map_is_refused_without_bus_master() {
    let _rig = HbaRig::left_armed();
    refuse(Call::MmioMap);
    assert_eq!(open(AHCI).err(), Some(AhciError::BrokerCallFailed(-12)));
    refused_without_bus_master(&seen());
}

#[test]
fn a_port_still_running_after_the_reset_refuses_the_controller() {
    let rig = HbaRig::left_armed();
    rig.hba().quirks.cr_stuck = CrStuck::Always;
    assert_eq!(open(AHCI).err(), Some(AhciError::EnginesRunning));
    refused_without_bus_master(&seen());
    let said = nonos_libc::said();
    assert!(said.iter().any(|l| l.contains("stopping its engines")), "{said:?}");
    assert!(said.iter().any(|l| l.contains("port 0 still runs after the HBA reset")), "{said:?}");
}
