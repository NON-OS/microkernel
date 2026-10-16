// NONOS Operating System (AGPL-3.0-or-later)

//! Which adapters have a driver: only the chips one is spawned for, by the
//! driver's own id table.

use crate::driver_ids::has_driver;

/* Settings says a chip with no NONOS driver plainly instead of waiting on a
driver that will never answer: the inventory spawns iwlwifi for Intel wireless
and the RTL8821CE driver for 10ec:c821, and nothing else. */
#[test]
fn only_the_chips_a_driver_is_spawned_for_have_one() {
    assert!(has_driver(0x10EC, 0xC821));
    for intel in [0x095A, 0x24FD, 0x2526, 0x9DF0, 0x2723, 0x06F0, 0x2725, 0x51F0, 0x7AF0, 0x7F70] {
        assert!(has_driver(0x8086, intel), "{intel:#x}");
    }
    // The 3165 and Tiger Lake's AX201 are found by the driver (b614f22a).
    for intel in [0xA0F0, 0x3165, 0x3166] {
        assert!(has_driver(0x8086, intel), "{intel:#x}");
    }
    // Intel ids the iwlwifi driver leaves at once on: no adapter's. The chip
    // has no driver; it is not one that failed.
    for intel in [0x24F7, 0x24F8, 0x24F9, 0x24FA, 0x24FC] {
        assert!(!has_driver(0x8086, intel), "{intel:#x}");
    }
    // MediaTek MT7921/MT7922, Broadcom, Qualcomm ath10k/ath11k, RTL8822CE/BE.
    for (v, d) in [
        (0x14C3, 0x7961),
        (0x14C3, 0x0616),
        (0x14E4, 0x43A0),
        (0x168C, 0x003E),
        (0x17CB, 0x1103),
        (0x10EC, 0xC822),
        (0x10EC, 0xB822),
    ] {
        assert!(!has_driver(v, d), "{v:#x}:{d:#x}");
    }
}

/* The panel's Intel rule is the iwlwifi driver's own id table, every id: a
chip the driver takes has a driver, one it leaves at once on does not. */
#[test]
fn the_intel_rule_is_the_drivers_own_id_table() {
    for id in 0..=u16::MAX {
        assert_eq!(
            has_driver(0x8086, id),
            crate::iwlwifi_family::family_for_device(id).is_some(),
            "8086:{id:04x}"
        );
    }
}
