// NONOS Operating System (AGPL-3.0-or-later)
//! Which devices are taken: products[] exactly, its interface, and the
//! configuration Linux would choose.

use nonos_usbnet::Bind;

use crate::ax::bind;
use crate::ax::function::find_function;
use crate::ax::products::listed;
use crate::bound::found;
use crate::chip::Chip;
use crate::layout::{CLASS_CONFIG, CONFIG};

/// ax88179_178a.c products[], v6.6.
const LINUX_PRODUCTS: [(u16, u16); 13] = [
    (0x0b95, 0x1790),
    (0x0b95, 0x178a),
    (0x04b4, 0x3610),
    (0x2001, 0x4a00),
    (0x0df6, 0x0072),
    (0x04e8, 0xa100),
    (0x17ef, 0x304b),
    (0x050d, 0x0128),
    (0x0930, 0x0a13),
    (0x0711, 0x0179),
    (0x07c9, 0x000e),
    (0x07c9, 0x000f),
    (0x07c9, 0x0010),
];

#[test]
fn exactly_linux_products_are_listed() {
    for (v, p) in LINUX_PRODUCTS {
        assert!(listed(v, p), "{v:04x}:{p:04x}");
    }
    // The ASIX parts cdc_ether names, and neighbours of listed ones.
    for (v, p) in [(0x0b95, 0x2790), (0x0b95, 0x2791), (0x0b95, 0x1791), (0x07c9, 0x0011)] {
        assert!(!listed(v, p), "{v:04x}:{p:04x}");
    }
}

#[test]
fn the_vendor_interface_and_its_bulk_pipes_are_found() {
    let f = find_function(&found(0x0b95, 0x1790, 0xff, &[&CONFIG])).unwrap();
    assert_eq!((f.config, f.interface, f.alt), (1, 0, 0));
    let p = f.pipes;
    assert_eq!(
        (p.bulk_in, p.bulk_out, p.max_packet_in, p.max_packet_out),
        (0x82, 0x03, 1024, 1024)
    );
    assert_eq!((p.burst_in, p.burst_out), (3, 3));
}

#[test]
fn a_class_configuration_linux_would_choose_is_left_to_its_class_driver() {
    let class_dev = found(0x0b95, 0x1790, 0x00, &[&CONFIG, &CLASS_CONFIG]);
    assert_eq!(find_function(&class_dev), None);
    let vendor_dev = found(0x0b95, 0x1790, 0xff, &[&CONFIG, &CLASS_CONFIG]);
    assert!(find_function(&vendor_dev).is_some(), "vendor class: the first configuration");
    let only_vendor = found(0x0b95, 0x1790, 0x00, &[&CONFIG]);
    assert!(find_function(&only_vendor).is_some());
}

#[test]
fn unlisted_devices_and_devices_without_the_interface_are_not_ours() {
    let bus = Chip::default().bus();
    let other = found(0x0b95, 0x2790, 0xff, &[&CONFIG]);
    assert!(matches!(bind(bus.clone(), &other), Bind::NotOurs));
    let no_if = found(0x0b95, 0x1790, 0xff, &[&CLASS_CONFIG]);
    assert!(matches!(bind(bus.clone(), &no_if), Bind::NotOurs));
    assert!(bus.0.borrow().calls.is_empty(), "the device is not touched");
}
