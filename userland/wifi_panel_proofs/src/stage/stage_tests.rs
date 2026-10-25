// NONOS Operating System (AGPL-3.0-or-later)
//! W4: stage 9 is iwlwifi restarting its firmware; every code reads as one
//! stage, and a code past the last reads as unknown, never as ready.

use super::stage_kind::DriverStage;

#[test]
fn a_restart_reads_as_restarting_and_no_code_reads_as_another() {
    assert!(DriverStage::from_code(9) == DriverStage::Restarting);
    assert_eq!(DriverStage::Restarting.text(), "The card's firmware stopped; restarting it");
    let named: Vec<DriverStage> = (0..=9).map(DriverStage::from_code).collect();
    for (i, a) in named.iter().enumerate() {
        assert!(*a != DriverStage::Unknown, "code {i} is named");
        assert!(named[i + 1..].iter().all(|b| b != a), "code {i} is its own stage");
    }
    for code in 10..=u8::MAX {
        assert!(DriverStage::from_code(code) == DriverStage::Unknown, "code {code}");
    }
    assert!(DriverStage::from_code(0) == DriverStage::Ready);
}
