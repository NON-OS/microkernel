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

use super::power_report::{battery, record, Battery, BATTERY_PRESENT, CHARGING, PERCENT_MASK};

#[test]
fn reports_are_believed_while_fresh() {
    assert_eq!(battery(1_000), Battery::Unreported, "nothing reported yet");
    record(BATTERY_PRESENT | CHARGING | 57, 10_000);
    assert_eq!(battery(10_000), Battery::Percent(57));
    assert_eq!(battery(190_000), Battery::Percent(57), "exactly three minutes");
    assert_eq!(battery(190_001), Battery::Unreported, "stale");
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
