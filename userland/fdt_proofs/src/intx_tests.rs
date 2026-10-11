// NONOS Operating System (AGPL-3.0-or-later)
//! A host bridge's INTx wiring, from virt's real `interrupt-map`.

use crate::arch::fdt::find::pci::interrupt_map;
use crate::arch::fdt::Fdt;
use crate::fixtures::{RPI4B, VIRT_GICV2, VIRT_GICV3};

#[test]
fn virt_swizzles_four_spis_across_the_slots() {
    for blob in [VIRT_GICV3, VIRT_GICV2] {
        let map = interrupt_map(&Fdt::from_bytes(blob).unwrap()).unwrap().unwrap();
        assert_eq!((map.count, map.mask_hi, map.mask_pin, map.root_bus), (16, 0x1800, 7, 0));
        // Slot 0 INTA..INTD are SPIs 3..6; slot 1 starts one further on.
        let pins: Vec<_> = (1..=4).map(|p| map.route(0, 0, 0, p)).collect();
        assert_eq!(
            pins,
            [Some((35, false)), Some((36, false)), Some((37, false)), Some((38, false))]
        );
        assert_eq!(map.route(0, 1, 0, 1), Some((36, false)));
        assert_eq!(map.route(0, 5, 3, 1), Some((36, false)), "slot 5 masks to slot 1");
        assert_eq!(map.route(0, 3, 0, 4), Some((37, false)));
    }
}

#[test]
fn what_the_map_does_not_wire_is_not_routed() {
    let map = interrupt_map(&Fdt::from_bytes(VIRT_GICV3).unwrap()).unwrap().unwrap();
    assert_eq!(map.route(1, 0, 0, 1), None, "behind a bridge");
    assert_eq!(map.route(0, 0, 0, 0), None, "no pin");
    assert_eq!(map.route(0, 0, 0, 5), None);
    assert_eq!(map.route(0, 32, 0, 1), None);
    assert_eq!(interrupt_map(&Fdt::from_bytes(RPI4B).unwrap()), Ok(None), "no ECAM host");
}
