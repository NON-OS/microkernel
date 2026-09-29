// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../../../src/arch/x86_64/time/rtc/error.rs"]
pub mod error;

pub mod types;

pub use error::{RtcError, RtcResult};
pub use types::{RtcAlarm};
