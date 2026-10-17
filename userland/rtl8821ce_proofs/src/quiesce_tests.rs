// NONOS Operating System (AGPL-3.0-or-later)
//! A chip is quiesced before what it was given can be freed (plan/50 C2):
//! every engine `init_trx_cfg` turned on reads stopped by the time Bus Master
//! goes off, and whether the Bus Master write was taken is passed on.

use std::cell::{Cell, RefCell};

use crate::mac::quiesce;
use crate::regs::Mmio;

const REG_CR: usize = 0x0100;

/// A register file that keeps every write in order.
#[derive(Default)]
struct Card {
    cr: Cell<u8>,
    log: RefCell<Vec<&'static str>>,
}

impl Mmio for Card {
    fn read8(&self, off: usize) -> u8 {
        if off == REG_CR {
            self.cr.get()
        } else {
            0
        }
    }
    fn write8(&self, off: usize, v: u8) {
        if off == REG_CR {
            self.cr.set(v);
            self.log.borrow_mut().push("engines");
        }
    }
    fn read16(&self, _: usize) -> u16 {
        0
    }
    fn write16(&self, _: usize, _: u16) {}
    fn read32(&self, _: usize) -> u32 {
        0
    }
    fn write32(&self, _: usize, _: u32) {}
}

#[test]
fn the_engines_stop_before_bus_master_goes_off() {
    for taken in [true, false] {
        let card = Card { cr: Cell::new(0xFF), ..Card::default() };
        let got = quiesce(&card, || {
            assert_eq!(card.cr.get(), 0, "every engine stopped already");
            card.log.borrow_mut().push("bus master");
            taken
        });
        assert_eq!(got, taken, "the broker's answer passed on");
        assert_eq!(*card.log.borrow(), ["engines", "bus master"]);
    }
}
