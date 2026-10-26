// NONOS Operating System (AGPL-3.0-or-later)
//! Device registers move with the kernel: one to one until it runs high,
//! then at the high alias, and TTBR is given the root the walker reads.

use boot_tables_proofs::arch::aarch64::mmu::kernel_va::{io, runs_high};
use boot_tables_proofs::arch::aarch64::mmu::probe::KERNEL_SPACE_START;
use boot_tables_proofs::arch::aarch64::mmu::probe_high::{go_high_and_reach, root_addresses};

#[test]
fn devices_are_reached_one_to_one_then_at_the_high_alias() {
    let uart = 0x0900_0000;
    assert!(!runs_high());
    assert_eq!(io(uart), uart);
    assert_eq!(go_high_and_reach(uart), (uart + KERNEL_SPACE_START, true));
}

#[test]
fn ttbr_is_given_the_root_the_walker_reads() {
    let (ttbr, walked) = root_addresses();
    assert_eq!(ttbr, walked);
}
