// NONOS Operating System (AGPL-3.0-or-later)
//! The PCH flash controller found by where it sits, every window touching
//! the flash's refused, and the claim and map paths wired to both.

use super::guard::{is_flash_function, overlaps_flash, FLASH_WINDOW};

#[test]
fn the_flash_controller_is_known_by_its_place_not_its_id() {
    assert!(is_flash_function(0, 0, 31, 5));
    assert!(!is_flash_function(0, 0, 31, 4), "the SMBus host beside it");
    assert!(!is_flash_function(0, 0, 21, 0), "an LPSS SPI function");
    assert!(!is_flash_function(1, 0, 31, 5), "another segment");
}

#[test]
fn every_window_touching_the_flash_is_refused() {
    let (lo, hi) = FLASH_WINDOW;
    assert_eq!((lo, hi), (0xFF00_0000, 0x1_0000_0000));
    assert!(overlaps_flash(0xFFF0_0000, 0x1000), "inside");
    assert!(overlaps_flash(lo - 0x1000, 0x2000), "reaching in from below");
    assert!(overlaps_flash(0xFE00_0000, 0x0200_0000), "spanning it");
    assert!(overlaps_flash(hi - 1, u64::MAX), "a size that would wrap");
    assert!(!overlaps_flash(lo - 0x1000, 0x1000), "ending at its start");
    assert!(!overlaps_flash(hi, 0x1000), "above 4 GiB");
    assert!(!overlaps_flash(0xFED4_0000, 0x5000), "the TPM's window");
    assert!(!overlaps_flash(lo, 0), "nothing at all");
}

#[test]
fn the_claim_and_map_paths_consult_the_guard() {
    let claim = include_str!("../../../../src/hardware/broker/claim/claim.rs");
    let first = claim.find("is_firmware(device_id)").expect("claim asks");
    // The claims' lock is first taken in the entry (enter.rs).
    assert!(first < claim.find("super::enter::enter(").unwrap(), "before anything is held");
    let firmware = include_str!("../../../../src/hardware/broker/claim/firmware.rs");
    assert!(firmware.contains("is_flash_function(") && firmware.contains("overlaps_flash("));
    let map = include_str!("../../../../src/hardware/broker/mmio/msix_exclusion.rs");
    assert!(map.contains("firmware_flash::FLASH_WINDOW"), "every map keeps out of it");
    // The claim's errnos are mapped in device_errno.rs, beside the syscall: a
    // firmware refusal is EPERM, as an unconfined or out-of-scope claim is.
    let sys = include_str!("../../../../src/syscall/microkernel/device_errno.rs");
    let perm = "ClaimError::Unconfined | ClaimError::Refused | ClaimError::Firmware => ERRNO_PERM";
    assert!(sys.contains(perm) && sys.contains("ClaimError::OutOfScope => ERRNO_PERM"));
}
