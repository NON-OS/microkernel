// NONOS Operating System (AGPL-3.0-or-later)
//! The chip's own address is never read; the drawn one is served.

use nonos_usbnet::mock::drawn;
use nonos_usbnet::Nic;

use crate::chip::{Access, PLA};
use crate::helpers::bound;

#[test]
fn the_drawn_address_is_served_and_the_chips_own_never_read() {
    let (p, r) = bound(0x6000);
    assert_eq!(r.mac(), drawn().bytes());
    let log = p.chip.borrow().log.clone();
    let factory = |a: &Access| matches!(*a, Access::Read(PLA, 0xc000 | 0xc004 | 0xdd00 | 0xdd02));
    assert!(!log.iter().any(factory), "the chip's own address or efuse was read");
}
