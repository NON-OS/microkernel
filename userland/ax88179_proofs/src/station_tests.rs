// NONOS Operating System (AGPL-3.0-or-later)
//! The station address: drawn for the run and written to AX_NODE_ID,
//! which also sets the chip's own unicast filter; the address the chip
//! carries is never read, and a chip with none binds all the same.

use nonos_usbnet::mock::drawn;
use nonos_usbnet::{Bind, Nic};

use crate::ax::bind;
use crate::bound::{bound, found};
use crate::calls::{mac_r, mac_w};
use crate::chip::Chip;
use crate::layout::CONFIG;

/// An address from ASIX's own range, as a chip's EEPROM would hold one.
const FACTORY: [u8; 6] = [0x00, 0x0e, 0xc6, 0x87, 0x72, 0x01];

#[test]
fn the_address_on_the_wire_is_drawn_and_written_to_the_chip() {
    let (bus, nic) = bound(&Chip { node_id: FACTORY.to_vec(), ..Chip::default() });
    let mac = nic.mac();
    assert_ne!(mac, FACTORY);
    assert_eq!(mac[0] & 0x03, 0x02, "locally administered unicast");
    let calls = bus.0.borrow().calls.clone();
    assert!(calls.contains(&mac_w(0x10, &mac)), "AX_NODE_ID holds the drawn address");
    assert!(!calls.contains(&mac_r(0x10, 6)), "the factory address is never read");
}

#[test]
fn a_chip_with_no_address_of_its_own_binds_all_the_same() {
    for id in [vec![0u8; 6], vec![0x01, 0, 0x5e, 0, 0, 1], vec![0xff; 6], vec![2, 0, 0, 1]] {
        let bus = Chip { node_id: id.clone(), ..Chip::default() }.bus();
        let r = bind(bus, &found(0x0b95, 0x1790, 0xff, &[&CONFIG]), drawn());
        assert!(matches!(r, Bind::Ours(_)), "{id:02x?}");
    }
}
