// NONOS Operating System (AGPL-3.0-or-later)
//! Rows of the table by name: the cores the kernel meets, as they come.

use crate::cases::{ids, A72, A73, A75, ARM};
use crate::spectre::answer::{Answer, Firmware};
use crate::spectre::decide::decide;
use crate::spectre::plan::{Bhb, Meltdown, Plan, Ssbd, Why, V2};

const SILENT: Firmware =
    Firmware { wa1: Answer::Unknown, wa2: Answer::Unknown, wa3: Answer::Unknown };
const ALL_CALL: Firmware = Firmware { wa1: Answer::Call, wa2: Answer::Call, wa3: Answer::Call };

/// A Raspberry Pi 4's Cortex-A72 with no firmware to ask: the loop with
/// k = 8 is the only remedy, and v2 and store bypass are named as open.
#[test]
fn a72_without_firmware() {
    let plan = decide(&ids([0; 6], (ARM, A72)), &SILENT);
    let open = Plan {
        v2: V2::Vulnerable,
        bhb: Bhb::Loop(8),
        ssbd: Ssbd::Vulnerable,
        meltdown: Meltdown::NotAffected(Why::Core),
    };
    assert_eq!(plan, open);
}

/// The same core under TF-A that answers every workaround: one firmware
/// call covers v2 and BHB, and SSBD is turned on by firmware.
#[test]
fn a72_with_firmware() {
    let plan = decide(&ids([0; 6], (ARM, A72)), &ALL_CALL);
    assert_eq!((plan.v2, plan.bhb, plan.ssbd), (V2::Firmware, Bhb::Firmware, Ssbd::Firmware));
}

/// Cortex-A73 has no published loop: without WORKAROUND_3 it stays open.
#[test]
fn a73_needs_the_firmware_for_bhb() {
    assert_eq!(decide(&ids([0; 6], (ARM, A73)), &SILENT).bhb, Bhb::Vulnerable);
}

/// Cortex-A75: Meltdown, no E0PD, so the kernel's mapping is exposed.
#[test]
fn a75_is_exposed_to_meltdown() {
    assert_eq!(decide(&ids([0; 6], (ARM, A75)), &ALL_CALL).meltdown, Meltdown::Exposed);
    let with_e0pd = ids([0, 0, 0, 0, 0, 1], (ARM, A75));
    assert_eq!(decide(&with_e0pd, &SILENT).meltdown, Meltdown::E0pd);
}
