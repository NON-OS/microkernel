// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../../../src/arch/x86_64/time/rtc/bcd.rs"]
pub mod bcd;

#[path = "../../../../../../../../../src/arch/x86_64/time/rtc/calendar.rs"]
pub mod calendar;

#[path = "../../../../../../../../../src/arch/x86_64/time/rtc/constants.rs"]
pub mod constants;

#[path = "../../../../../../../../../src/arch/x86_64/time/rtc/conversion.rs"]
pub mod conversion;

#[path = "../../../../../../../../../src/arch/x86_64/time/rtc/error.rs"]
pub mod error;

pub mod types;

#[path = "../../../../../../../../../src/arch/x86_64/time/rtc/unix.rs"]
pub mod unix;

pub use bcd::is_valid_bcd;
pub use calendar::day_of_year;
pub use constants::Register;
pub use conversion::{bcd_to_bin, bin_to_bcd, datetime_to_unix, day_name, day_of_week, days_in_month, is_leap_year, month_name, unix_to_datetime};
pub use error::{RtcError, RtcResult};
pub use types::{RtcTime};
