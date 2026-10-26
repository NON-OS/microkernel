// NONOS Operating System (AGPL-3.0-or-later)

//! The one switch for power save.

/// Leisure power save, with ASPM L1 while the firmware sleeps. Off: proven on
/// the host only. Turning it on needs the release run on a real RTL8821CE:
/// the link held through sleep, no received frame lost across it, and a group
/// rekey and an SA Query answered while asleep.
pub const LPS: bool = false;
