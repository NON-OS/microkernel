// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/process/signal/set/groups.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod process;

pub fn signalset_standard_signals() -> crate::process::signal::set::bits::SignalSet {
    crate::process::signal::set::bits::SignalSet::standard_signals()
}

pub fn signalset_realtime_signals() -> crate::process::signal::set::bits::SignalSet {
    crate::process::signal::set::bits::SignalSet::realtime_signals()
}

pub fn signalset_uncatchable() -> crate::process::signal::set::bits::SignalSet {
    crate::process::signal::set::bits::SignalSet::uncatchable()
}

