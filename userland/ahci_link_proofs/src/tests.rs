// NONOS Operating System (AGPL-3.0-or-later)
//! The link predicates gate command issue. A command must not be issued until
//! the PHY reports DET==3 and the device has cleared BSY and DRQ; these pin the
//! exact bit decode against the SATA status and task-file registers.

use crate::engine::link::established::link_established;
use crate::engine::link::ready::device_ready;

#[test]
fn link_is_established_only_when_det_is_three() {
    assert!(link_established(0x3)); // present, PHY communication established
    assert!(link_established(0x0000_0113)); // upper SSTS fields ignored, DET==3
    assert!(!link_established(0x0)); // no device
    assert!(!link_established(0x1)); // present, no communication yet
    assert!(!link_established(0x4)); // PHY offline
}

#[test]
fn device_is_ready_only_when_bsy_and_drq_are_clear() {
    assert!(device_ready(0x00)); // idle
    assert!(device_ready(0x40)); // an unrelated status bit set, BSY/DRQ clear
    assert!(!device_ready(0x80)); // BSY set
    assert!(!device_ready(0x08)); // DRQ set
    assert!(!device_ready(0x88)); // both set
}

// The status a give-up leaves with: a controller with no disk on
// any port is a clean EXIT_EMPTY, every other give-up stays EXIT_GAVE_UP.
#[path = "../../capsule_driver_ahci/src/exit_code.rs"]
mod exit_code;

#[test]
fn a_controller_with_no_disk_leaves_empty_not_given_up() {
    use crate::error::AhciError;
    use exit_code::exit_code;
    use nonos_libc::bringup::{EXIT_ABSENT, EXIT_EMPTY, EXIT_GAVE_UP};
    assert_eq!(exit_code(EXIT_GAVE_UP, Some(AhciError::NoDisk)), EXIT_EMPTY);
    assert_eq!(exit_code(EXIT_GAVE_UP, Some(AhciError::Timeout)), EXIT_GAVE_UP, "a failing disk");
    assert_eq!(exit_code(EXIT_GAVE_UP, None), EXIT_GAVE_UP);
    assert_eq!(exit_code(EXIT_ABSENT, Some(AhciError::NoDisk)), EXIT_ABSENT, "no controller");
    assert_eq!((EXIT_EMPTY, EXIT_GAVE_UP), (3, 6));
}

#[test]
fn the_driver_leaves_with_the_code_its_last_attempt_gives() {
    let main = include_str!("../../capsule_driver_ahci/src/main.rs");
    assert!(main.contains("last = Some(e);"), "the last failure is not kept");
    assert!(main.contains("Err(code) => mk_exit(exit_code::exit_code(code, last)),"));
}
