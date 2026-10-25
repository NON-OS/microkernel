// NONOS Operating System (AGPL-3.0-or-later)
//! Where every actor is: the APs, the targets' handlers and the initiator.

use super::config::MAX_CPUS;
use super::types::Val;

/// Where an AP is. The descriptor reads `Offline` for `Offline` and
/// `Abandoned`, `Starting` for `Starting`, `Woken`, `Entered` and `Parked`,
/// and `Online` for `Online`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Ap {
    Offline,
    /// `configure_descriptor` stored `Starting`; INIT not yet delivered.
    Starting,
    /// INIT and STARTUP delivered: running the trampoline on the kernel's
    /// CR3 with an empty TLB, before its claim.
    Woken,
    /// Claimed its start (`ap_entry`); at step `n` of `bring_up`, interrupts
    /// masked. At its spin points (`round`'s lock loop, `lock_responsive`) it
    /// serves its own pending flag.
    Entered(u8),
    /// Asked to park, it parked itself at a checkpoint: `cli; hlt`, its
    /// claim word `Parked`. Still `Starting` to everyone else until the boot
    /// CPU has seen that and INIT has reset it.
    Parked,
    /// `publish` stored `Online`; interrupts on.
    Online,
    /// The boot CPU took the claim, stored `Offline` and sent INIT: halted,
    /// TLB empty, never running kernel code again.
    Abandoned,
}

/// The target side of one round, from the pending flag to the ack.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Handler {
    Idle,
    /// Took its pending flag (`swap(false)`).
    Taken,
    /// Has done one of flush and ack, and owes the other.
    Half,
}

/// Where the initiator is in `flush_tlb_*_smp` and `broadcast`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Init {
    Start,
    Written,
    LocalFlushed,
    /// Reading CPU `n`'s descriptor for `select`.
    Selecting(u8),
    /// Setting the pending flag of the `n`th CPU and sending to it.
    Sending(u8),
    Waiting,
    Done,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct State {
    pub pte: Val,
    pub init: Init,
    pub targets: u8,
    pub acks: u8,
    pub tlb: [Option<Val>; MAX_CPUS],
    pub ap: [Ap; MAX_CPUS],
    pub pending: [bool; MAX_CPUS],
    pub handler: [Handler; MAX_CPUS],
    /// The boot CPU stored `AbandonRequested` in this AP's claim word: it
    /// missed its online deadline.
    pub park_asked: [bool; MAX_CPUS],
}
