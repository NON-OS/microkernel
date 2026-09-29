// NONOS Operating System (AGPL-3.0-or-later)

pub mod pit;

pub use pit::{AccessMode as PitAccessMode, Channel as PitChannel, Mode as PitMode, PitError, PitStatistics, PIT_FREQUENCY};
