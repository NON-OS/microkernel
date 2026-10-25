// NONOS Operating System (AGPL-3.0-or-later)
//! W4: the 8821C masters the bus only once its power cycle showed the MAC
//! off (or it was cold), CARD_ENABLE ran and its registers answer. A chip
//! whose power-off did not take, whose power-on failed or whose registers
//! read dead never gets Bus Master; a refused write is named.

use super::armed::{armed, Armed};
use crate::pwr::{power_on_then_master, NoMaster, PowerOn};

/// Run the gate with a Bus Master write that `takes`, logged in order.
fn run(a: &Armed, takes: bool) -> Result<(u32, PowerOn), NoMaster> {
    power_on_then_master(a, || {
        assert!(!a.armed.get(), "the engines are off before Bus Master");
        a.log.borrow_mut().push("bus_master");
        takes
    })
}

#[test]
fn a_warm_card_masters_only_after_its_power_off_took() {
    let a = armed(true, true);
    let got = run(&a, true);
    assert!(matches!(got, Ok((0x21, PowerOn::WasLeftPowered { cr_after_off: 0xEA }))));
    assert_eq!(*a.log.borrow(), ["powered_off", "bus_master"]);
}

#[test]
fn a_warm_card_whose_power_off_did_not_take_never_masters() {
    let a = armed(true, false);
    assert_eq!(run(&a, true).err(), Some(NoMaster::LeftOn { cr_after_off: 0x00 }));
    assert!(a.log.borrow().is_empty(), "no Bus Master write");
}

#[test]
fn a_cold_card_masters_after_its_power_on() {
    let a = armed(false, false);
    assert!(matches!(run(&a, true), Ok((0x21, PowerOn::FromCold))));
    assert_eq!(*a.log.borrow(), ["bus_master"]);
}

#[test]
fn a_failed_power_on_or_dead_registers_never_master() {
    let a = armed(false, false);
    a.chip.regs.borrow_mut()[0x06] = 0x00; // power-ready never comes
    assert_eq!(run(&a, true).err(), Some(NoMaster::PowerFailed));
    assert!(a.log.borrow().is_empty());
    for dead in [0x0000_0000u32, 0xFFFF_FFFF] {
        let a = armed(true, true);
        a.chip.regs.borrow_mut()[0xFC..0x100].copy_from_slice(&dead.to_le_bytes());
        assert_eq!(run(&a, true).err(), Some(NoMaster::DeadMmio), "{dead:#x}");
        assert_eq!(*a.log.borrow(), ["powered_off"], "powered off, but no Bus Master");
    }
}

#[test]
fn a_refused_bus_master_write_is_named() {
    let a = armed(true, true);
    assert_eq!(run(&a, false).err(), Some(NoMaster::Refused));
    assert_eq!(*a.log.borrow(), ["powered_off", "bus_master"], "asked once, after the stop");
}
