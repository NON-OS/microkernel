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

use crate::arch::x86_64::uefi::tables::time::EfiTime;

fn midnight(year: u16, month: u8, day: u8) -> EfiTime {
    EfiTime { year, month, day, timezone: EfiTime::TIMEZONE_UNSPECIFIED, ..EfiTime::default() }
}

#[test]
fn days_past_the_end_of_the_month_are_invalid() {
    assert!(!midnight(2021, 2, 31).is_valid());
    assert!(!midnight(2023, 2, 29).is_valid());
    assert!(!midnight(2100, 2, 29).is_valid());
    assert!(!midnight(2024, 4, 31).is_valid());
    assert!(midnight(2024, 2, 29).is_valid());
    assert!(midnight(2000, 2, 29).is_valid());
    assert!(midnight(2024, 12, 31).is_valid());
}

#[test]
fn dates_before_1970_count_back_from_the_epoch() {
    assert_eq!(midnight(1970, 1, 1).to_unix_timestamp(), 0);
    assert_eq!(midnight(1969, 12, 31).to_unix_timestamp(), -86_400);
    assert_eq!(midnight(1900, 1, 1).to_unix_timestamp(), -2_208_988_800);
    assert_eq!(midnight(1970, 12, 31).to_unix_timestamp(), 31_449_600);
    assert_eq!(midnight(2000, 3, 1).to_unix_timestamp(), 951_868_800);
}
