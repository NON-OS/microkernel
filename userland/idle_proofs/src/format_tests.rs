// NONOS Operating System (AGPL-3.0-or-later)
//! A power_state is standby exactly when its StateType bit is clear: bit 16
//! in the original format, bit 30 in the extended one, and no other bit.

use crate::format::is_standby;

#[test]
fn only_the_state_type_bit_decides() {
    for bit in 0..32 {
        let param = 1u32 << bit;
        assert_eq!(is_standby(param, false), bit != 16, "original, bit {bit}");
        assert_eq!(is_standby(param, true), bit != 30, "extended, bit {bit}");
        assert!(!is_standby(param | 1 << 16, false));
        assert!(!is_standby(param | 1 << 30, true));
    }
}

#[test]
fn the_formats_disagree_where_they_should() {
    // A power level and StateID in the original format, powerdown there; in
    // the extended format bit 16 is StateID, so the same word is standby.
    let original_powerdown = 0x0101_0000;
    assert!(!is_standby(original_powerdown, false));
    assert!(is_standby(original_powerdown, true));
    // Extended powerdown: bit 30 is StateID in the original format.
    let extended_powerdown = 0x4000_0003;
    assert!(!is_standby(extended_powerdown, true));
    assert!(is_standby(extended_powerdown, false));
    assert!(is_standby(0, false) && is_standby(0, true));
}
