// NONOS Operating System (AGPL-3.0-or-later)

//! Each join's address: drawn once a link, one per network, written to the
//! MAC-ID registers, and never changed under an association.

use std::cell::RefCell;
use std::rc::Rc;

use super::card::NoRx;
use super::rig::{Dma, AP, STA};
use crate::link::{NoAddress, RtlLink};
use crate::regs::Mmio;

const HOME: [u8; 6] = [0x12, 0xA5, 0x21, 0x69, 0xAE, 0x1D]; // core proofs' vector

/// A card that keeps every 32-bit write.
#[derive(Clone, Default)]
struct Log(Rc<RefCell<Vec<(usize, u32)>>>);

impl Mmio for Log {
    fn read8(&self, _: usize) -> u8 {
        0
    }
    fn write8(&self, _: usize, _: u8) {}
    fn read16(&self, _: usize) -> u16 {
        0
    }
    fn write16(&self, _: usize, _: u16) {}
    fn read32(&self, _: usize) -> u32 {
        0
    }
    fn write32(&self, off: usize, v: u32) {
        self.0.borrow_mut().push((off, v));
    }
}

fn link(log: &Log) -> RtlLink<Log, Dma, NoRx> {
    let dma = |len: usize, at: u64| Dma(Rc::new(RefCell::new(vec![0u8; len])), at);
    RtlLink::new(log.clone(), dma(4096, 0x1000), dma(1 << 16, 0x2000), dma(4096, 0x3000), NoRx, STA)
}

#[test]
fn each_network_gets_its_own_address_from_one_key_drawn_once() {
    let log = Log::default();
    let mut l = link(&log);
    assert_eq!(l.readdress(b"home", || Some([0x5A; 32])), Ok(HOME));
    let id = |w: &(usize, u32)| w.0 == 0x0610 || w.0 == 0x0614;
    let written: Vec<_> = log.0.borrow().iter().copied().filter(id).collect();
    assert_eq!(written, vec![(0x0610, 0x6921_A512), (0x0614, 0x1DAE)], "MAC-ID registers");
    let work = l.readdress(b"work", || panic!("the key is drawn once")).unwrap();
    assert_ne!(work, HOME);
    assert_eq!(l.readdress(b"home", || None), Ok(HOME), "the same network, the same address");
}

#[test]
fn no_randomness_no_address_and_an_association_keeps_its_own() {
    let log = Log::default();
    let mut l = link(&log);
    assert_eq!(l.readdress(b"home", || None), Err(NoAddress::NoKey));
    assert!(log.0.borrow().iter().all(|w| w.0 != 0x0610), "nothing written");
    assert_eq!(l.readdress(b"home", || Some([0x5A; 32])), Ok(HOME));
    l.associate(AP, [0; 16]);
    assert_eq!(l.readdress(b"work", || None), Err(NoAddress::Associated));
    assert_eq!(nonos_wifi_core::netif::LinkPort::mac(&l), Some(HOME), "sent under HOME");
}
