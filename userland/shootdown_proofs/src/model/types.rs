// NONOS Operating System (AGPL-3.0-or-later)
//! The values the model is built from.

/// What a TLB entry for the page says, or what the page table says.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Val {
    Old,
    New,
}

/// How a target drops the page: `invlpg` for a range, or a CR3 reload for
/// `pages == 0` (`handle.rs`).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Flush {
    Page,
    Full,
}

/// One step of an AP's `bring_up`, as far as parking is concerned.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Step {
    /// The AP checks whether the boot CPU asked it to park, and parks there.
    Checkpoint,
    /// Takes a lock (the paging manager in `arm_ap_guards`, the allocator).
    Lock,
    /// Releases it.
    Unlock,
}

/// `bring_up` as the kernel has it: a checkpoint before and after each step
/// that takes a lock, and none inside one.
pub const BRING_UP: &[Step] = &[
    Step::Checkpoint,
    Step::Lock,
    Step::Unlock,
    Step::Checkpoint,
    Step::Lock,
    Step::Unlock,
    Step::Checkpoint,
];
