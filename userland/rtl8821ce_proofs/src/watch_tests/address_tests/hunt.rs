// NONOS Operating System (AGPL-3.0-or-later)

//! A hidden network's hunt runs under a one-off address, heard under, then
//! left for the network's own.

use super::super::rig::AP;
use super::rig::{link, Log, HOME};
use nonos_wifi_core::address::Security::Psk;

#[test]
fn a_hunt_address_is_heard_under_then_left_for_the_networks() {
    let log = Log::default();
    let mut l = link(&log);
    assert_eq!(l.address_ready(|| Some([0x5A; 32])), Ok(()));
    assert_eq!(l.take_address([0x06, 1, 2, 3, 4, 5]), Ok(()));
    assert_eq!(log.0.borrow().last(), Some(&(0x0614, 0x0504)), "answers to it are heard");
    assert_eq!(l.readdress(b"home", Psk), Ok(HOME));
    l.associate(AP, [0; 16]);
    assert_eq!(nonos_wifi_core::netif::LinkPort::mac(&l), Some(HOME), "the join runs under HOME");
}
