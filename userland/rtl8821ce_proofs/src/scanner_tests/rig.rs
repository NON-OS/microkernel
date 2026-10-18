// NONOS Operating System (AGPL-3.0-or-later)
//! The frames a station might answer, and DMA memory a test can read back.

use std::cell::RefCell;
use std::rc::Rc;

use crate::fw::dma::DmaMem;

pub(super) const STA: [u8; 6] = [0x02, 0x11, 0x22, 0x33, 0x44, 0x55];
pub(super) const AP: [u8; 6] = [0x02, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE];

#[derive(Clone)]
pub(super) struct Dma(pub(super) Rc<RefCell<Vec<u8>>>, pub(super) u64);

impl Dma {
    pub(super) fn new(len: usize, dev: u64) -> Self {
        Self(Rc::new(RefCell::new(vec![0; len])), dev)
    }
}

impl DmaMem for Dma {
    fn capacity(&self) -> usize {
        self.0.borrow().len()
    }
    fn device_addr(&self) -> u64 {
        self.1
    }
    fn write_bytes(&self, offset: usize, src: &[u8]) {
        self.0.borrow_mut()[offset..offset + src.len()].copy_from_slice(src);
    }
}

/// A management frame of `subtype` to `da` from the access point.
fn mgmt(subtype: u8, da: [u8; 6], body: &[u8]) -> Vec<u8> {
    let mut f = vec![subtype << 4, 0, 0, 0];
    [da, AP, AP].iter().for_each(|a| f.extend_from_slice(a));
    f.extend_from_slice(&[0x10, 0x00]);
    f.extend_from_slice(body);
    f
}

pub(super) fn provoking() -> Vec<Vec<u8>> {
    let beacon = [&[0u8; 8][..], &[0x64, 0, 0x01, 0x04, 0, 4], b"home", &[3, 1, 6]].concat();
    let eapol = [
        &[0x08, 0x02, 0, 0][..],
        &STA,
        &AP,
        &AP,
        &[0x20, 0],
        &[0xAA, 0xAA, 3, 0, 0, 0, 0x88, 0x8E],
        &[2, 3, 0, 95],
    ]
    .concat();
    // Each as the chip hands it up, its FCS still on.
    let frames = vec![
        mgmt(8, [0xFF; 6], &beacon),
        mgmt(4, STA, &[0, 4, b'h', b'o', b'm', b'e']),
        mgmt(4, [0xFF; 6], &[0, 0]),
        mgmt(11, STA, &[0, 0, 2, 0, 0, 0]),
        mgmt(12, STA, &[7, 0]),
        mgmt(13, STA, &[8, 0, 0x12, 0x34]),
        eapol,
    ];
    frames.into_iter().map(|f| [f, vec![0xFC; 4]].concat()).collect()
}
