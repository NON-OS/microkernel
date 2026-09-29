// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../../../src/arch/x86_64/time/pit/access.rs"]
pub mod access;

#[path = "../../../../../../../../../src/arch/x86_64/time/pit/channel.rs"]
pub mod channel;

#[path = "../../../../../../../../../src/arch/x86_64/time/pit/channel_state.rs"]
pub mod channel_state;

#[path = "../../../../../../../../../src/arch/x86_64/time/pit/command.rs"]
pub mod command;

#[path = "../../../../../../../../../src/arch/x86_64/time/pit/constants.rs"]
pub mod constants;

#[path = "../../../../../../../../../src/arch/x86_64/time/pit/conversion.rs"]
pub mod conversion;

#[path = "../../../../../../../../../src/arch/x86_64/time/pit/error.rs"]
pub mod error;

#[path = "../../../../../../../../../src/arch/x86_64/time/pit/mode.rs"]
pub mod mode;

#[path = "../../../../../../../../../src/arch/x86_64/time/pit/ports.rs"]
pub mod ports;

#[path = "../../../../../../../../../src/arch/x86_64/time/pit/stats.rs"]
pub mod stats;

#[path = "../../../../../../../../../src/arch/x86_64/time/pit/types.rs"]
pub mod types;

pub use constants::{DEFAULT_FREQUENCY, MAX_DIVISOR, MAX_TIMER_FREQUENCY, MIN_DIVISOR, MIN_TIMER_FREQUENCY, PIT_FREQUENCY};
pub use conversion::{divisor_to_frequency, divisor_to_period_ns, frequency_error, frequency_to_divisor, period_us_to_divisor};
pub use types::{AccessMode, Channel, Mode, PitError, PitResult, PitStatistics};
