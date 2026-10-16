// NONOS Operating System (AGPL-3.0-or-later)
//! A port that came up, handed back. On a controller given back whole the
//! port is stopped, then Bus Master written off, then the release, which
//! frees the port's regions: none goes back by hand while Bus Master is on.
//! On the controller kept, the port served keeps Bus Master on, so a port
//! parked there goes back by hand, stopped first.

use nonos_libc::{bus_master_after_reset, bus_master_off, first_bus_master, released_last};
use nonos_libc::{seen, Call};

use super::open::open;
use super::open::Opened;
use super::open_tests::AHCI;
use crate::engine::init::init_port;
use crate::engine::park::{park, retire};
use crate::engine::Port;
use crate::model::HbaRig;

/// The controller open, and its port 0 up and running.
pub(super) fn port_up(rig: &HbaRig) -> (Opened, Port) {
    let opened = open(AHCI).expect("the controller opens");
    let cap = opened.info.cap;
    let port = init_port(AHCI.device_id, opened.epoch, opened.regs, cap, 0).expect("port 0 up");
    assert_ne!(rig.hba().engines().running, 0, "the port runs");
    (opened, port)
}

#[test]
fn a_controller_given_back_whole_stops_its_ports_before_bus_master_and_the_release() {
    let rig = HbaRig::left_armed();
    let (opened, port) = port_up(&rig);
    retire(port, opened.regs);
    drop(opened);
    let seen = seen();
    bus_master_after_reset(&seen);
    assert_eq!(seen.iter().filter(|s| s.call == Call::DmaMap).count(), 4, "{seen:?}");
    let (_, off) = bus_master_off(&seen);
    assert_eq!(off.running, 0, "Bus Master off with the port running: {seen:?}");
    released_last(&seen);
}

#[test]
fn a_port_parked_on_the_controller_kept_is_stopped_before_its_regions_go() {
    let rig = HbaRig::left_armed();
    let (opened, port) = port_up(&rig);
    park(port, opened.regs);
    let seen = seen();
    let gone: Vec<_> = seen.iter().filter(|s| s.call == Call::DmaUnmap).collect();
    assert_eq!(gone.len(), 4, "{seen:?}");
    assert!(gone.iter().all(|s| s.device.running == 0), "a region went first: {seen:?}");
    assert!(first_bus_master(&seen).is_some(), "Bus Master stays on for the port served");
    drop(opened);
}
