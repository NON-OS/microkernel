// NONOS Operating System (AGPL-3.0-or-later)

//! How long a hunt pumps, and how it ends.

/// The pause between pumps of the receive queue while hunting.
pub(super) const PUMP_US: u32 = 10_000;

/// How long a stopped sweep may take to report its completion.
pub(super) const ABORT_PUMPS: u32 = 100;

/// Why the hunt found nothing.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HuntEnd {
    /// A whole sweep heard this many beacons, none a candidate.
    NotHeard(u32),
    /// The sweep could not run, or would not stop.
    Radio,
}
