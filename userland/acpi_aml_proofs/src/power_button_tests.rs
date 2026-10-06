// NONOS Operating System (AGPL-3.0-or-later)
//! The fixed-feature power button: which FADTs it can be polled on, and
//! what is read and written.

use crate::arch::x86_64::acpi::hw::fadt_decode::*;
use crate::arch::x86_64::acpi::hw::gas::*;
use crate::arch::x86_64::acpi::hw::power_button::*;
use crate::fadt_tests::{gemini_lake_fadt, FadtBuilder};

fn decoded(bytes: &[u8]) -> FadtInfo {
    decode_fadt(bytes).expect("FACP")
}

#[test]
fn gemini_lake_has_a_fixed_button_at_the_pm1a_status_port() {
    let f = decoded(&gemini_lake_fadt());
    assert_eq!(classify(&f), Button::Fixed { sts_a: 0x400, sts_b: 0 });
}

#[test]
fn a_control_method_button_is_left_to_aml() {
    let mut f = decoded(&gemini_lake_fadt());
    f.flags |= FADT_PWR_BUTTON;
    assert_eq!(classify(&f), Button::ControlMethod);
}

#[test]
fn a_hardware_reduced_platform_has_no_pm1_button() {
    let mut f = decoded(&gemini_lake_fadt());
    f.flags |= FLAG_HW_REDUCED_ACPI;
    assert_eq!(classify(&f), Button::HwReduced);
}

#[test]
fn a_memory_mapped_pm1_block_is_not_polled_from_the_tick() {
    let mut f = decoded(&gemini_lake_fadt());
    f.pm1a_evt = Gas { space: SPACE_SYSTEM_MEMORY, bit_width: 32, bit_offset: 0, access_size: 2, address: 0xFED0_3000 };
    assert_eq!(classify(&f), Button::Unreachable);
    f.pm1a_evt = Gas::empty();
    assert_eq!(classify(&f), Button::Unreachable, "no PM1 event block at all");
}

#[test]
fn a_second_pm1_block_is_polled_too() {
    let mut f = decoded(&gemini_lake_fadt());
    f.pm1b_evt = Gas { space: SPACE_SYSTEM_IO, bit_width: 32, bit_offset: 0, access_size: 2, address: 0x500 };
    assert_eq!(classify(&f), Button::Fixed { sts_a: 0x400, sts_b: 0x500 });
}

#[test]
fn an_acpi1_fadt_with_legacy_ports_classifies_from_them() {
    // A 116-byte ACPI 1.0 FADT: PM1a at 0x8000 from the 32-bit field.
    let bytes = FadtBuilder::new(116)
        .u32(off::PM1A_EVT_BLK, 0x8000)
        .u32(off::PM1A_CNT_BLK, 0x8004)
        .u8(off::PM1_EVT_LEN, 4)
        .u8(off::PM1_CNT_LEN, 2)
        .sealed();
    assert_eq!(classify(&decoded(&bytes)), Button::Fixed { sts_a: 0x8000, sts_b: 0 });
}

#[test]
fn only_the_power_button_bit_counts_and_only_it_is_cleared() {
    assert!(pressed(0x0100));
    assert!(pressed(0x8101), "WAK_STS and the timer bit set beside it");
    assert!(!pressed(0x8001), "other status bits are not a press");
    assert_eq!(clear_value(), 0x0100, "write-one-to-clear: every other bit written as zero");
}

#[test]
fn sci_en_says_whether_the_os_owns_the_button() {
    assert!(acpi_mode(0x0001));
    assert!(acpi_mode(0x1C01));
    assert!(!acpi_mode(0x1C00), "legacy mode: the button belongs to the SMI handler");
}

#[test]
fn a_held_or_bouncing_button_is_one_press_per_second() {
    assert!(fresh(None, 0));
    assert!(!fresh(Some(1_000), 1_500));
    assert!(!fresh(Some(1_000), 1_999));
    assert!(fresh(Some(1_000), 2_000));
    assert!(fresh(Some(1_000), u64::MAX));
}
