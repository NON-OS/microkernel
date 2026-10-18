// NONOS Operating System (AGPL-3.0-or-later)
//! What the device sends unasked: a media state kept as the link, and a
//! KEEPALIVE answered, read once its notification endpoint says a response
//! waits (looked at once every 64 receives); and their bytes checked.

use nonos_usbnet::mock::Call;
use nonos_usbnet::{Bind, Nic, Setup};

use crate::control_tests::bind_with;
use crate::device::{indicate, words};
use crate::rndis::from_device::{from_device, keepalive_done, FromDevice};

const SEND: Setup = Setup::new(0x21, 0x00, 0, 0);
const AVAILABLE: [u8; 8] = [1, 0, 0, 0, 0, 0, 0, 0];

#[test]
fn media_states_and_keepalives_are_read_and_their_lengths_checked() {
    assert_eq!(from_device(&indicate(0x4001_000B)), FromDevice::Media(true));
    assert_eq!(from_device(&indicate(0x4001_000C)), FromDevice::Media(false));
    assert_eq!(from_device(&words(&[8, 12, 77])), FromDevice::KeepAlive(77));
    assert_eq!(from_device(&words(&[8, 16, 77])), FromDevice::Other, "longer than what came");
    assert_eq!(from_device(&words(&[8, 8, 77])), FromDevice::Other, "shorter than a message");
    assert_eq!(keepalive_done(77).to_vec(), words(&[0x8000_0008, 16, 77, 0]));
}

/// QEMU's usb-net, with `extra` queued behind the packet filter's
/// completion, the second SET's (the station address is the first), so it
/// waits for the first response read after bind.
fn bound_with(
    extra: Vec<u8>,
) -> (nonos_usbnet::mock::MockBus, crate::rndis::link::Rndis<nonos_usbnet::mock::MockBus>) {
    let (mut extra, mut sets) = (Some(extra), 0);
    let (bus, r) = bind_with(Box::new(move |kind, c| {
        sets += (kind == 5) as u32;
        match (kind, sets) {
            (5, 2) => [vec![c], extra.take().into_iter().collect()].concat(),
            _ => vec![c],
        }
    }));
    let Bind::Ours(nic) = r else { panic!("not bound") };
    (bus, nic)
}

#[test]
fn a_media_disconnect_takes_the_link_down() {
    let (bus, mut nic) = bound_with(indicate(0x4001_000C));
    assert!(nic.notify.is_some() && nic.link_up(), "up from bind");
    bus.0.borrow_mut().notices.push_back(AVAILABLE.to_vec());
    let mut out = [0u8; 1514];
    for _ in 0..64 {
        let _ = nic.recv(&mut out);
    }
    assert!(!nic.link_up());
}

#[test]
fn a_keepalive_is_answered_under_its_request_id() {
    let (bus, mut nic) = bound_with(words(&[8, 12, 77]));
    bus.0.borrow_mut().calls.clear();
    bus.0.borrow_mut().notices.push_back(AVAILABLE.to_vec());
    let mut out = [0u8; 1514];
    for _ in 0..64 {
        let _ = nic.recv(&mut out);
    }
    let answered = Call::Out(SEND, keepalive_done(77).to_vec());
    assert!(bus.0.borrow().calls.contains(&answered));
    assert!(nic.link_up(), "a keepalive changes nothing else");
}
