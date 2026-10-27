// NONOS Operating System (AGPL-3.0-or-later)

//! Which functions the driver takes, and which chip each id is: Linux
//! v6.12's `mt7921_pci_device_table`, and `mt7921_pci_probe`'s MT7920 bit.

use crate::ids::{supported, Chip, PCI_IDS, VENDOR_ITTIM, VENDOR_MEDIATEK};

#[test]
fn the_table_is_linuxs_six_ids() {
    let mut ids = PCI_IDS.to_vec();
    ids.sort_unstable();
    let mut want = vec![
        (VENDOR_MEDIATEK, 0x7961),
        (VENDOR_MEDIATEK, 0x7922),
        (VENDOR_ITTIM, 0x7922),
        (VENDOR_MEDIATEK, 0x0608),
        (VENDOR_MEDIATEK, 0x0616),
        (VENDOR_MEDIATEK, 0x7920),
    ];
    want.sort_unstable();
    assert_eq!(ids, want);
    assert_eq!((VENDOR_MEDIATEK, VENDOR_ITTIM), (0x14c3, 0x0b48));
}

#[test]
fn the_t14_gen_4s_card_and_the_rz608_are_taken() {
    assert!(supported(0x14c3, 0x0616), "MT7922, AMD RZ616");
    assert!(supported(0x14c3, 0x0608), "MT7921, AMD RZ608");
    assert!(!supported(0x14c3, 0x7925), "MT7925 is another driver's");
    assert!(!supported(0x8086, 0x0616), "the device id alone is not enough");
}

#[test]
fn the_chip_id_names_the_chip_and_bound_bit_7_the_mt7920() {
    assert_eq!(Chip::from_ids(0x7961, 0), Some(Chip::Mt7961));
    assert_eq!(Chip::from_ids(0x7961, 1 << 7), Some(Chip::Mt7920));
    assert_eq!(Chip::from_ids(0x7922, 1 << 7), Some(Chip::Mt7922), "bit 7 is the 7961's alone");
    assert_eq!(Chip::from_ids(0xffff_7922, 0), Some(Chip::Mt7922), "the low half is the id");
    assert_eq!(Chip::from_ids(0x7925, 0), None);
    assert_eq!(Chip::from_ids(u32::MAX, u32::MAX), None, "a dead card is no chip");
}
