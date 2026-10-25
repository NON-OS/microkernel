// NONOS Operating System (AGPL-3.0-or-later)
//! The rules under test, and the bound on machine size.

use super::types::{Flush, Pge, Step, BRING_UP};

/// The rules under test. `Config::kernel()` is the code; every other value is
/// a mutant that removes or reorders one rule.
#[derive(Clone, Copy, Debug)]
pub struct Config {
    /// Number of CPUs, the initiator included.
    pub cpus: usize,
    /// APs already online when the round starts; the rest are offline and may
    /// be started, or started and given up on, at any point during it.
    pub online_at_start: usize,
    pub flush: Flush,
    /// `select` targets a CPU whose descriptor reads `Starting` as well as
    /// `Online`. The boot CPU writes `Starting` before it sends INIT, so a
    /// CPU `select` reads as `Offline` has an empty TLB from after the write.
    /// The mutant targets `Online` alone, as the code did.
    pub target_starting: bool,
    /// The waiter acknowledges for a target the boot CPU gave up on and set
    /// back to `Offline`: INIT emptied its TLB, and it will never answer.
    pub ack_for_abandoned: bool,
    /// The boot CPU may ask an AP that entered to park, at any point of its
    /// bring-up. Off, the search covers the round without that path, which
    /// keeps machines of four and five CPUs within memory.
    pub may_ask_park: bool,
    /// The steps of `bring_up`, where an AP the boot CPU asked to park may
    /// park.
    pub bring_up: &'static [Step],
    /// An AP the boot CPU asked to park leaves the target set only once it
    /// has parked and INIT has reset it. The mutant drops it at the request,
    /// while it may still be running with what it cached.
    pub drop_after_init: bool,
    /// The initiator writes the page table before it selects targets.
    pub write_before_select: bool,
    /// A target flushes before it acknowledges.
    pub flush_before_ack: bool,
    /// Which CPUs have CR4.PGE set. Kernel PTEs are GLOBAL.
    pub pge: Pge,
    /// The full flush toggles PGE, which drops GLOBAL entries too. The mutant
    /// is a CR3 write alone, which keeps them on a CPU with PGE set.
    pub full_toggles_pge: bool,
}

impl Config {
    pub const fn kernel(cpus: usize, online_at_start: usize, flush: Flush) -> Self {
        Config {
            cpus,
            online_at_start,
            flush,
            target_starting: true,
            ack_for_abandoned: true,
            may_ask_park: true,
            bring_up: BRING_UP,
            drop_after_init: true,
            write_before_select: true,
            flush_before_ack: true,
            pge: Pge::Every,
            full_toggles_pge: true,
        }
    }
}

pub const MAX_CPUS: usize = 5;
