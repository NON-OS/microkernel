// NONOS Operating System (AGPL-3.0-or-later)
//! A scripted AX88179 behind MockBus: register reads answered with the
//! values a test sets, every call recorded, one named request refused.

use std::cell::Cell;
use std::rc::Rc;

use nonos_usbnet::mock::{Call, MockBus};
use nonos_usbnet::Setup;

/// A locally administered unicast address, made up for the tests.
pub const MAC: [u8; 6] = [0x02, 0x00, 0x00, 0x17, 0x90, 0x01];
/// BMCR with auto-negotiation enabled (bit 12) and full duplex, 1000.
pub const BMCR: u16 = 0x1140;

pub struct Chip {
    pub node_id: Vec<u8>,
    pub physr: Rc<Cell<u16>>,
    pub link_sts: u8,
    pub eeprom: Option<u16>,
    pub fifo_busy: bool,
    /// A request refused: bRequest, wValue, wIndex, and the errno.
    pub refuse: Option<(u8, u16, u16, i32)>,
}

impl Default for Chip {
    fn default() -> Self {
        let physr = Rc::new(Cell::new(0));
        let (node_id, link_sts) = (MAC.to_vec(), 0x04);
        Self { node_id, physr, link_sts, eeprom: None, fifo_busy: false, refuse: None }
    }
}

impl Chip {
    pub fn bus(&self) -> MockBus {
        let (id, physr, sts) = (self.node_id.clone(), self.physr.clone(), self.link_sts);
        let (eeprom, busy, refuse) = (self.eeprom, self.fifo_busy, self.refuse);
        MockBus::new(Box::new(move |call| {
            let key = |s: &Setup| (s.request, s.value, s.index);
            match (call, refuse) {
                (Call::In(s, _) | Call::Out(s, _), Some((r, v, i, e))) if key(s) == (r, v, i) => {
                    Err(e)
                }
                (Call::In(s, _), _) => match key(s) {
                    (0x01, 0x10, 6) => Ok(id.clone()),
                    (0x01, 0x02, 1) => Ok(vec![sts]),
                    (0x01, 0x33, 1) => Ok(vec![0x03]),
                    (0x01, 0x26, 2) => Ok(vec![0x20, 0x00]),
                    (0x02, 3, 0x11) => Ok(physr.get().to_le_bytes().to_vec()),
                    (0x02, 3, 0x00) => Ok(BMCR.to_le_bytes().to_vec()),
                    (0x04, 0x43, 1) => eeprom.map(|w| w.to_le_bytes().to_vec()).ok_or(-32),
                    (0x81, 0x8c, 0) => Ok(((busy as u32) << 30).to_le_bytes().to_vec()),
                    _ => Err(-32),
                },
                _ => Ok(vec![]),
            }
        }))
    }
}
