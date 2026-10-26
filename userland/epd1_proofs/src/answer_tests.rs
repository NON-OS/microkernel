// NONOS Operating System (AGPL-3.0-or-later)
//! What SMCCC_ARCH_FEATURES returns reads as the decision takes it (DEN0028):
//! 0, call the workaround; 1 or NOT_REQUIRED (-2), none is needed; anything
//! else, or no SMCCC 1.1 to ask, the firmware cannot say.

use crate::spectre::answer::Answer;

#[test]
fn each_firmware_return_reads_as_den0028_says() {
    assert_eq!(Answer::from_ret(Some(0)), Answer::Call);
    assert_eq!(Answer::from_ret(Some(1)), Answer::NotRequired);
    assert_eq!(Answer::from_ret(Some(-2)), Answer::NotRequired);
    for ret in [Some(-1), Some(2), Some(i64::MIN), None] {
        assert_eq!(Answer::from_ret(ret), Answer::Unknown, "{ret:?}");
    }
}
