// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/process/signal/set/bits.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod process;

pub fn signalset_empty() -> crate::process::signal::set::bits::SignalSet {
    crate::process::signal::set::bits::SignalSet::empty()
}

pub fn signalset_full() -> crate::process::signal::set::bits::SignalSet {
    crate::process::signal::set::bits::SignalSet::full()
}

pub fn signalset_contains(this: crate::process::signal::set::bits::SignalSet, signo: u8) -> bool {
    this.contains(signo)
}

pub fn signalset_is_empty(this: crate::process::signal::set::bits::SignalSet) -> bool {
    this.is_empty()
}

pub fn signalset_as_bits(this: crate::process::signal::set::bits::SignalSet) -> u64 {
    this.as_bits()
}

pub fn signalset_from_bits(bits: u64) -> crate::process::signal::set::bits::SignalSet {
    crate::process::signal::set::bits::SignalSet::from_bits(bits)
}

pub fn signalset_complement(this: crate::process::signal::set::bits::SignalSet) -> crate::process::signal::set::bits::SignalSet {
    this.complement()
}

pub fn signalset_count(this: crate::process::signal::set::bits::SignalSet) -> usize {
    this.count()
}

