// NONOS Operating System (AGPL-3.0-or-later)
//! W4: stage 9 is iwlwifi restarting its firmware; 10 and 11 the MediaTek
//! driver's firmware package, missing or not matching; every code reads as
//! one stage, and a code past the last reads as unknown, never as ready.

use super::stage_kind::DriverStage;

#[test]
fn a_restart_reads_as_restarting_and_no_code_reads_as_another() {
    assert!(DriverStage::from_code(9) == DriverStage::Restarting);
    assert_eq!(DriverStage::Restarting.text(), "The card's firmware stopped; restarting it");
    let named: Vec<DriverStage> = (0..=11).map(DriverStage::from_code).collect();
    for (i, a) in named.iter().enumerate() {
        assert!(*a != DriverStage::Unknown, "code {i} is named");
        assert!(named[i + 1..].iter().all(|b| b != a), "code {i} is its own stage");
    }
    for code in 12..=u8::MAX {
        assert!(DriverStage::from_code(code) == DriverStage::Unknown, "code {code}");
    }
    assert!(DriverStage::from_code(0) == DriverStage::Ready);
}

#[test]
fn the_firmware_package_stages_say_what_is_wrong_with_it() {
    assert!(DriverStage::from_code(10) == DriverStage::PackageMissing);
    assert!(DriverStage::from_code(11) == DriverStage::PackageMismatch);
    assert_eq!(
        DriverStage::PackageMissing.text(),
        "The firmware package is missing from the boot medium"
    );
    assert_eq!(
        DriverStage::PackageMismatch.text(),
        "The firmware package does not match this build"
    );
}
