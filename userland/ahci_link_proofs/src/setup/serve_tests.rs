// NONOS Operating System (AGPL-3.0-or-later)
//! `serve` over two controllers open on one model HBA of three ports, each
//! port its own: controller 0 has port 0 up, controller 1 ports 1 and 2, and
//! port 1 holds the store magic. Controller 0 goes back whole: its port
//! stopped, Bus Master off, the release, none of its regions unmapped by
//! hand. On controller 1, kept, port 2 is parked, stopped before its regions
//! go, while port 1 is served and keeps running.

use nonos_libc::{bus_master_off, seen, Call};

use super::open::{open, Opened};
use super::open_tests::AHCI;
use super::serve::serve;
use super::walked::{Probed, Walk};
use crate::choose::Candidate;
use crate::discover::Found;
use crate::engine::init::init_port;
use crate::model::HbaRig;

/// Port `index` of `ctl`, up, the walk's candidate for controller `at`.
fn up(ctl: &Opened, device: u64, at: usize, index: u8, store: bool) -> Probed {
    let port = init_port(device, ctl.epoch, ctl.regs, ctl.info.cap, index).expect("port up");
    Probed { port, cand: Candidate { controller: at, port: index, store, plan: false } }
}

#[test]
fn serve_gives_a_controller_not_kept_back_whole_and_parks_the_kept_ones_others() {
    let _rig = HbaRig::with_ports(3);
    let first = open(AHCI).expect("controller 0 opens");
    let second = open(Found { device_id: 10, ..AHCI }).expect("controller 1 opens");
    let probed =
        vec![up(&first, 9, 0, 0, false), up(&second, 10, 1, 1, true), up(&second, 10, 1, 2, false)];
    let served = probed[1].port.clb.device_addr();
    let walk =
        Walk { opened: vec![first, second], probed, first_error: None, first_port_error: None };
    let driver = serve(walk).expect("the disk with the store is served");
    let seen = seen();
    let release = seen.iter().position(|s| s.call == Call::Release).expect("controller 0 goes");
    let gone: Vec<_> = seen.iter().filter(|s| s.call == Call::DmaUnmap).collect();
    assert_eq!(gone.len(), 4, "port 2's regions only: {seen:?}");
    assert!(gone.iter().all(|s| s.device.ports[2] == 0 && s.device.ports[1] != 0), "{seen:?}");
    assert!(seen[..release].iter().all(|s| s.call != Call::StaleUnmap), "{seen:?}");
    let (_, off) = bus_master_off(&seen);
    assert_eq!((off.ports[0], off.ports[2]), (0, 0), "off with a port given back running");
    assert_ne!(off.ports[1], 0, "the served port stopped: {seen:?}");
    assert_eq!(driver.block.as_ref().map(|p| p.clb.device_addr()), Some(served));
    let (pi, port, irq) = (driver.info.pi, driver.ports[1], driver.handles.irq_grant_id());
    assert_eq!((pi, port.index, port.implemented, irq), (0b111, 1, 1, 0x41), "controller 1 kept");
    let before = seen.len();
    drop(driver);
    let after = nonos_libc::seen();
    assert!(after[before..].iter().all(|s| s.call != Call::StaleUnmap && s.call != Call::DmaUnmap));
    assert_eq!(after.last().map(|s| (s.call, s.device.ports[1])), Some((Call::Release, 0)));
}
