// NONOS Operating System (AGPL-3.0-or-later)

//! Power save on the serving radio: a disconnect wakes it before saying
//! goodbye to the access point.

use nonos_wifi_core::netif::LinkPort;

use super::connect::disconnect;
use super::host_radio::{host_radio, Host};
use super::radio::{Radio, RadioUp};
use crate::pcie::Aspm;
use crate::ps::{Ps, PERIOD_MS};
use crate::tests_all::linkport_tests::rig::AP_MAC;

pub(super) const T0: u64 = 1_000;

pub(super) fn beacon() -> Vec<u8> {
    let head = [&[0x80, 0, 0, 0][..], &[0xFF; 6], &AP_MAC, &AP_MAC, &[0, 0]].concat();
    [head, vec![0; 12], vec![0, 3], b"net".to_vec(), vec![0; 4]].concat()
}

// Joined, the firmware told, the access point heard: asleep after a period.
pub(super) fn asleep(up: &mut RadioUp, host: &mut Host) {
    up.link.associate(AP_MAC, [0x5A; 16]);
    let mut ps = Ps::new(Aspm::default());
    ps.force_on();
    ps.assume_ready();
    up.link.set_ps(ps, None);
    host.stage(beacon().len() as u32, &beacon());
    up.link.tick(T0);
    up.link.tick(T0 + PERIOD_MS - 1);
    let _ = up.link.poll_rx(&mut [0u8; 2048]);
    up.link.tick(T0 + PERIOD_MS);
    assert!(up.link.ps().asleep(), "asleep first");
}

#[test]
fn a_disconnect_wakes_a_sleeping_radio() {
    let (mut radio, mut host) = host_radio();
    let Radio::Up(up) = &mut radio else { unreachable!() };
    asleep(up, &mut host);
    disconnect(&mut up.link, &mut up.keys, &up.regs, &mut None);
    assert!(!up.link.ps().asleep(), "awake once disconnected");
}
