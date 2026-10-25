// NONOS Operating System (AGPL-3.0-or-later)
//! Arming the polled power button: the rule against the QEMU model, and a
//! mutant for each half of it.

use crate::arch::x86_64::acpi::hw::fadt_decode::decode_fadt;
use crate::arch::x86_64::acpi::hw::gas::{Gas, SPACE_SYSTEM_MEMORY};
use crate::arch::x86_64::acpi::hw::power_button_arm::*;
use crate::fadt_tests::gemini_lake_fadt;
use crate::power_button_arm_model::{check, SCI_SOURCES};

#[test]
fn arming_masks_the_sci_then_sets_the_enable_keeping_every_other() {
    for en in 0..=u16::MAX {
        let steps = arm(false, en).expect("armed");
        assert_eq!(steps, [ArmStep::MaskSci, ArmStep::Enable(en | PM1_EN_PWRBTN)]);
        assert_eq!(check(&steps, en & !SCI_SOURCES), Ok(()), "en {en:#06x}");
    }
}

#[test]
fn a_capsule_holding_the_sci_is_left_its_line_and_enables() {
    assert_eq!(arm(true, 0), None);
    assert_eq!(arm(true, PM1_EN_PWRBTN), None);
}

#[test]
fn mutant_enable_not_set_leaves_a_qemu_press_unseen() {
    assert_eq!(
        check(&[ArmStep::MaskSci, ArmStep::Enable(0)], 0),
        Err("a press after arming was not latched")
    );
    assert_eq!(
        check(&[], 0),
        Err("a press after arming was not latched"),
        "the code before arming"
    );
}

#[test]
fn mutant_sci_left_unmasked_raises_an_unhandled_sci() {
    let unmasked = [ArmStep::Enable(PM1_EN_PWRBTN)];
    assert_eq!(check(&unmasked, 0), Err("an SCI was raised with its line unmasked"));
    let late = [ArmStep::Enable(PM1_EN_PWRBTN), ArmStep::MaskSci];
    assert_eq!(
        check(&late, 0),
        Err("an SCI was raised with its line unmasked"),
        "enable before mask"
    );
}

#[test]
fn the_enable_port_is_the_second_half_of_the_event_block() {
    let f = decode_fadt(&gemini_lake_fadt()).expect("FACP");
    assert_eq!(io_enable_port(&f.pm1a_evt), Some(0x402));
    assert_eq!(io_enable_port(&Gas::io(0x600, 4)), Some(0x602), "QEMU q35's PMBASE");
    let mmio = Gas {
        space: SPACE_SYSTEM_MEMORY,
        bit_width: 32,
        bit_offset: 0,
        access_size: 2,
        address: 0xFED0_3000,
    };
    assert_eq!(io_enable_port(&mmio), None);
    assert_eq!(io_enable_port(&Gas::empty()), None);
}
