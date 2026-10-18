// NONOS Operating System (AGPL-3.0-or-later)
//! A Command write the broker refuses, the claim's Memory Space write or the
//! Bus Master write after the reset: the controller is released either way,
//! Bus Master never on.

use nonos_libc::{refuse, refused_without_bus_master, seen, Call, Seen};
use nonos_libc::{MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_MEMORY_SPACE};

use super::open::open;
use super::open_tests::AHCI;
use crate::error::AhciError;
use crate::model::HbaRig;

/// Open a controller whose broker refuses every Command write carrying `bits`.
fn refused(bits: u16) -> Vec<Seen> {
    let _rig = HbaRig::left_armed();
    refuse(Call::Command(bits));
    assert_eq!(open(AHCI).err(), Some(AhciError::BrokerCallFailed(-16)));
    seen()
}

#[test]
fn a_memory_space_write_the_broker_refuses_releases_the_claim() {
    let seen = refused(MK_PCI_CMD_MEMORY_SPACE);
    assert!(seen.iter().all(|s| s.call != Call::MmioMap), "mapped after the refusal: {seen:?}");
    refused_without_bus_master(&seen);
}

#[test]
fn a_bus_master_write_the_broker_refuses_releases_with_bus_master_never_on() {
    let seen = refused(MK_PCI_CMD_BUS_MASTER);
    let on = |s: &&Seen| matches!(s.call, Call::Command(w) if w & MK_PCI_CMD_BUS_MASTER != 0);
    let asked: Vec<_> = seen.iter().filter(on).collect();
    assert!(asked.len() == 1 && asked[0].refused, "asked once, and refused: {seen:?}");
    refused_without_bus_master(&seen);
}
