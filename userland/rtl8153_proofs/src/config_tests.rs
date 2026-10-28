// NONOS Operating System (AGPL-3.0-or-later)
//! Which ids and which configuration: the sixteen adapters, each checked
//! against Linux v6.12's r8152 table by hand, and a first interface of any
//! class but the vendor's left alone.

use nonos_usbnet::Bind;

use crate::adapter::CONFIG_VENDOR;
use crate::chip::plug_with;
use crate::helpers::try_bind;
use crate::rtl::adapters::listed;

const FAMILY: [(u16, u16); 16] = [
    (0x0bda, 0x8153),
    (0x17ef, 0x3062),
    (0x17ef, 0x3069),
    (0x17ef, 0x3082),
    (0x17ef, 0x7205),
    (0x17ef, 0x720c),
    (0x17ef, 0x7214),
    (0x17ef, 0x721e),
    (0x17ef, 0xa387),
    (0x045e, 0x07ab),
    (0x045e, 0x07c6),
    (0x045e, 0x0927),
    (0x2357, 0x0601),
    (0x13b1, 0x0041),
    (0x04e8, 0xa101),
    (0x0955, 0x09ff),
];

#[test]
fn the_listed_ids_are_the_sixteen_and_no_other_realtek_part() {
    for (v, p) in FAMILY {
        assert!(listed(v, p), "{v:04x}:{p:04x}");
    }
    for (v, p) in [(0x0bda, 0x8152), (0x0bda, 0x8156), (0x0bda, 0x8050), (0x2001, 0xb301)] {
        assert!(!listed(v, p), "{v:04x}:{p:04x}");
    }
}

#[test]
fn a_first_interface_of_another_class_is_not_the_vendor_configuration() {
    for class in [0x02, 0x0a, 0xe0] {
        let mut config = CONFIG_VENDOR.to_vec();
        config[14] = class;
        let p = plug_with(0x5c30, (0x0bda, 0x8153), vec![config]);
        assert!(matches!(try_bind(&p), Bind::NotOurs), "class {class:02x}");
    }
}
