// NONOS Operating System (AGPL-3.0-or-later)
//! The entry sequence cannot run at EL0, so its order is read from the
//! source: what the arm64 boot protocol needs, in the order it needs it.

use crate::source::{code, order};

const ENTER: &str = "arch/aarch64/asm/enter_kernel.S";

#[test]
fn masks_then_data_cache_off_then_walk_then_mmu_off_then_branch() {
    order(
        &code(ENTER),
        &[
            "nonos_arm_enter_kernel:",
            "msr     daifset, #0xf",
            "sctlr_clear 0x4",
            "clean_all_by_set_way",
            "sctlr_clear 0x1001",
            "ic      iallu",
            "dsb     nsh",
            "isb",
            "mov     x0, x20",
            "mov     x1, x21",
            "br      x19",
        ],
    );
}

#[test]
fn sctlr_is_cleared_at_the_el_it_runs_at() {
    let src = code(ENTER);
    order(
        &src,
        &[
            ".macro sctlr_clear",
            "mrs     x9, CurrentEL",
            "cmp     x9, #(2 << 2)",
            "b.eq    80f",
            "msr     sctlr_el1, x10",
            "80: mrs     x10, sctlr_el2",
            "msr     sctlr_el2, x10",
            "81: isb",
            ".endm",
        ],
    );
}
