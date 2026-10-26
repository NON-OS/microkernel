// NONOS Operating System (AGPL-3.0-or-later)
//! What a test may do to the kernel's move to the high half, beside `probe`.

use super::{kernel_va, state};

/// The kernel moving to the high half (`kernel_va::devices_high`): where it
/// then reaches the registers at `pa`, and whether it says it runs high.
pub fn go_high_and_reach(pa: u64) -> (u64, bool) {
    kernel_va::devices_high();
    (kernel_va::io(pa), kernel_va::runs_high())
}

/// The root table's physical address, as TTBR is given it (`state::l0_addr`),
/// and the root the walker reads.
pub fn root_addresses() -> (u64, u64) {
    // SAFETY: both only read where the static root lives.
    unsafe { (state::l0_addr(), core::ptr::from_ref(state::l0()) as u64) }
}
