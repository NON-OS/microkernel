// NONOS Operating System (AGPL-3.0-or-later)
//! The link as the device reports it: NETWORK_CONNECTION on the
//! notification endpoint, looked at once every 64 receives; up from bind
//! until the device says otherwise.

use nonos_usbnet::{Bind, Nic};

use crate::device::Dev;
use crate::ncm::bind;
use crate::ncm::notice::connected;

fn notice(up: bool) -> Vec<u8> {
    vec![0xA1, 0x00, up as u8, 0, 0, 0, 0, 0]
}

#[test]
fn network_connection_is_read_and_others_are_not() {
    assert_eq!((connected(&notice(true)), connected(&notice(false))), (Some(true), Some(false)));
    let speed = [0xA1, 0x2A, 0, 0, 0, 0, 8, 0];
    assert_eq!((connected(&speed), connected(&notice(true)[..7])), (None, None));
}

#[test]
fn the_link_follows_the_device_from_its_notification_endpoint() {
    let dev = Dev::ncm(0);
    let (found, bus) = (dev.found(), dev.bus());
    let Bind::Ours(mut nic) = bind(bus.clone(), &found) else { panic!("not bound") };
    assert_eq!(nic.notify, Some(3), "interrupt IN 0x81, DCI 3");
    assert!(nic.link_up(), "up from bind");
    bus.0.borrow_mut().notices.push_back(notice(false));
    let mut out = [0u8; 1514];
    for _ in 0..63 {
        let _ = nic.recv(&mut out);
    }
    assert!(nic.link_up(), "not looked at yet");
    let _ = nic.recv(&mut out);
    assert!(!nic.link_up(), "down, as the device said");
    bus.0.borrow_mut().notices.push_back(vec![0xA1, 0x2A, 0, 0, 0, 0, 8, 0]);
    bus.0.borrow_mut().notices.push_back(notice(true));
    for _ in 0..128 {
        let _ = nic.recv(&mut out);
    }
    assert!(nic.link_up(), "a speed change leaves it; the next connection raises it");
}
