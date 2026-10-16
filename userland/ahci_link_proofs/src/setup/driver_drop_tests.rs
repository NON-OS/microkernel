// NONOS Operating System (AGPL-3.0-or-later)
//! The served controller at `Driver` drop: the served port stopped, then Bus
//! Master written off, then the release, which frees the port's regions.
//! None goes back by hand, before the release or again after it.

use nonos_libc::{seen, Call, MK_PCI_CMD_MEMORY_SPACE};

use super::give_back_tests::port_up;
use super::Driver;
use crate::constants::regs::{CMD_FRE, CMD_ST};
use crate::model::HbaRig;

#[test]
fn a_driver_dropped_stops_its_port_and_gives_each_grant_back_once() {
    let rig = HbaRig::left_armed();
    let (opened, port) = port_up(&rig);
    let (handles, regs, info, ports) = (opened.handles, opened.regs, opened.info, opened.ports);
    let driver = Driver { handles, regs, info, ports, block: Some(port) };
    let before = seen().len();
    drop(driver);
    let seen = seen();
    let tail: Vec<Call> = seen[before..].iter().map(|s| s.call).collect();
    let back = [Call::Command(MK_PCI_CMD_MEMORY_SPACE), Call::IrqUnbind, Call::MmioUnmap];
    assert_eq!(tail, [&back[..], &[Call::Release]].concat(), "{seen:?}");
    let at_release = seen.last().expect("released").device;
    assert_eq!(at_release.running & (CMD_ST | CMD_FRE), 0, "ST or FRE on at the release");
}
