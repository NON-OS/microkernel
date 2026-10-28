// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! What MkBatteryStatus believes: nothing before a report or once it is
//! three minutes old; a percentage, absent or present-unknown otherwise.
//! One test, since the report is one static the cases would share.

use super::power_report::{
    battery, record, state, Battery, BATTERY_PRESENT, CHARGING, PERCENT_MASK,
};

const BATTERY: &str = include_str!("../../../../src/syscall/microkernel/battery.rs");
const LIBC: &str = include_str!("../../../libc/src/battery.rs");

/// MkBatteryStatus's forms: 0 the percentage, 1 the word, others EINVAL;
/// libc's state call asks for form 1 in the first register.
#[test]
fn the_forms_are_the_ones_libc_asks_for() {
    let flat = |s: &str| s.split_whitespace().collect::<String>();
    let arms = "FORM_PERCENT=>report::answer(),FORM_STATE=>report::state(),_=>returnERRNO_INVAL,";
    assert!(flat(BATTERY).contains(arms), "each form to its answer");
    assert!(flat(BATTERY).contains("pubconstFORM_STATE:u64=1;"));
    assert!(flat(LIBC).contains("call_raw(N_MK_BATTERY_STATUS,[1,0,0,0,0,0])"));
    assert!(flat(LIBC).contains("call_raw(N_MK_BATTERY_STATUS,[0;6])"), "form 0 as before");
}

#[test]
fn reports_are_believed_while_fresh() {
    assert_eq!((battery(1_000), state(1_000)), (Battery::Unreported, None), "nothing yet");
    record(BATTERY_PRESENT | CHARGING | 57, 10_000);
    assert_eq!(battery(10_000), Battery::Percent(57));
    assert_eq!(battery(190_000), Battery::Percent(57), "exactly three minutes");
    // MkBatteryStatus form 1: the whole word, as long as the percentage.
    assert_eq!(state(190_000), Some(BATTERY_PRESENT | CHARGING | 57));
    assert_eq!((battery(190_001), state(190_001)), (Battery::Unreported, None), "stale");
    record(0, 200_000);
    assert_eq!(battery(200_500), Battery::Absent);
    record(BATTERY_PRESENT | PERCENT_MASK, 300_000);
    assert_eq!(battery(300_000), Battery::Present, "present, charge unknown");
    record(BATTERY_PRESENT | 101, 400_000);
    assert_eq!(battery(400_000), Battery::Present, "a charge above 100 is not believed");
    record(BATTERY_PRESENT, 0);
    assert_eq!(
        battery(1),
        Battery::Percent(0),
        "a report at boot's first millisecond still counts"
    );
}
