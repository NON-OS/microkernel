// NONOS Operating System (AGPL-3.0-or-later)
//! Start-up speaks under this driver's name: the shared claim's reasons
//! renamed, any other kept; the give-back lines whole, tagged, and naming
//! no address.

use crate::pwr::NoMaster;
use crate::start_say::{own, refusal};

#[test]
fn the_shared_claim_reasons_are_renamed() {
    assert_eq!(own("rtl8821ce: claim failed"), "rtl8822ce: claim failed");
    assert_eq!(
        own("rtl8821ce: memory space enable failed"),
        "rtl8822ce: memory space enable failed"
    );
    assert_eq!(own("rtl8821ce: mmio map failed"), "rtl8822ce: mmio map failed");
    assert_eq!(own("something else"), "something else");
}

#[test]
fn the_give_back_lines_are_whole_and_tagged() {
    let all = [
        NoMaster::PowerFailed,
        NoMaster::LeftOn { cr_after_off: 1 },
        NoMaster::DeadMmio,
        NoMaster::Refused,
    ];
    for why in all {
        let line = refusal(why);
        assert!(line.starts_with(b"[rtl8822ce] "), "{why:?}");
        assert_eq!(line.iter().filter(|&&b| b == b'\n').count(), 1);
        assert_eq!(line.last(), Some(&b'\n'));
    }
}
