// NONOS Operating System (AGPL-3.0-or-later)
//! The offsets `asm/percpu.h` gives the vectors are the block's own: the
//! user key, the kernel key and the on word, in the block's `pac` words.

use crate::percpu::layout::{CPU_ID, KERNEL_STACK_TOP, PAC, SELF_PTR, USER_STACK_SAVED};
use crate::percpu::PerCpuData;

const HEADER: &str = include_str!("../../../src/arch/aarch64/asm/percpu.h");

fn define(name: &str) -> usize {
    let line = HEADER.lines().find(|l| l.split_whitespace().nth(1) == Some(name));
    line.and_then(|l| l.split_whitespace().nth(2)?.parse().ok()).expect(name)
}

#[test]
fn the_header_names_the_blocks_pac_words() {
    assert_eq!(define("PERCPU_PAC_USER"), PAC);
    assert_eq!(define("PERCPU_PAC_KERNEL"), PAC + 16);
    assert_eq!(define("PERCPU_PAC_ON"), PAC + 32);
    let block = PerCpuData::new();
    let base = &block as *const PerCpuData as usize;
    assert_eq!(&block.pac[0] as *const _ as usize - base, PAC);
    assert_eq!(&block.pac[4] as *const _ as usize - base, PAC + 32);
}

#[test]
fn the_pac_words_leave_the_fields_x86_reads_by_offset_alone() {
    for fixed in [SELF_PTR, CPU_ID, KERNEL_STACK_TOP, USER_STACK_SAVED] {
        assert!(fixed + 8 <= PAC, "{fixed:#x}");
    }
    assert_eq!(core::mem::size_of::<PerCpuData>(), 4096);
    assert_eq!(PAC % 8, 0, "ldp needs the words 8-aligned");
}
