// NONOS Operating System (AGPL-3.0-or-later)

//! How long the hunt listens on each channel, and how many sweeps it makes.

/// Milliseconds of uptime to dwell on each channel while hunting the target's
/// beacon: two and a half beacon intervals (102.4 ms each), so one beacon
/// lost to noise or a busy medium does not hide the network. At 120 ms one
/// miss read as "not heard on any channel" on the HP.
pub(super) const BEACON_HUNT_DWELL_MS: i64 = 250;

/// Full sweeps before the network is called not heard.
pub(super) const HUNT_SWEEPS: usize = 2;
