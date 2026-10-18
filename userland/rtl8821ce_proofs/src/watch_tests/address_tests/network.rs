// NONOS Operating System (AGPL-3.0-or-later)

//! One address per network, a name under one security, from one key drawn
//! once a link; never changed under an association.

use super::super::rig::AP;
use super::rig::{link, Log, HOME};
use crate::link::NoAddress;
use nonos_wifi_core::address::Security::{Psk, Sae};

#[test]
fn each_network_gets_its_own_address_from_one_key_drawn_once() {
    let log = Log::default();
    let mut l = link(&log);
    assert_eq!(l.address_ready(|| Some([0x5A; 32])), Ok(()));
    assert_eq!(l.readdress(b"home", Psk), Ok(HOME));
    let id = |w: &(usize, u32)| w.0 == 0x0610 || w.0 == 0x0614;
    let written: Vec<_> = log.0.borrow().iter().copied().filter(id).collect();
    assert_eq!(written, vec![(0x0610, 0x4E48_149E), (0x0614, 0x0D38)], "MAC-ID registers");
    assert_eq!(l.address_ready(|| panic!("the key is drawn once")), Ok(()));
    assert_ne!(l.readdress(b"home", Sae).unwrap(), HOME, "SAE home is another network");
    assert_ne!(l.readdress(b"work", Psk).unwrap(), HOME);
    assert_eq!(l.readdress(b"home", Psk), Ok(HOME), "the same network, the same address");
}

#[test]
fn no_randomness_no_address_and_an_association_keeps_its_own() {
    let log = Log::default();
    let mut l = link(&log);
    assert_eq!(l.address_ready(|| None), Err(NoAddress::NoKey));
    assert_eq!(l.readdress(b"home", Psk), Err(NoAddress::NoKey));
    assert!(log.0.borrow().iter().all(|w| w.0 != 0x0610), "nothing written");
    assert_eq!(l.address_ready(|| Some([0x5A; 32])), Ok(()));
    assert_eq!(l.readdress(b"home", Psk), Ok(HOME));
    l.associate(AP, [0; 16]);
    assert_eq!(l.address_ready(|| None), Err(NoAddress::Associated));
    assert_eq!(l.readdress(b"work", Psk), Err(NoAddress::Associated));
    assert_eq!(l.take_address([0x02; 6]), Err(NoAddress::Associated));
    assert_eq!(nonos_wifi_core::netif::LinkPort::mac(&l), Some(HOME), "sent under HOME");
}
