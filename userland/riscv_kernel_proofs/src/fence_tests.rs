// NONOS Operating System (AGPL-3.0-or-later)
//! Each riscv64 MMIO and DMA barrier names the FENCE sets its contract needs
//! (RISC-V unprivileged ISA, FENCE: i device input, o device output, r and w
//! memory): read from the kernel's source.

use crate::source::{code, order};

const MMIO: &str = "arch/riscv64/mmio_order";

#[test]
fn block_barriers_order_what_they_say() {
    let f = code(&format!("{MMIO}/fences.rs"));
    order(
        &f,
        &[
            "fn fence_writes()",
            "fence ow, ow",
            "fn fence_reads()",
            "fence ir, ir",
            "fn fence_full()",
            "fence iorw, iorw",
        ],
    );
}

#[test]
fn an_acquire_read_fences_after_and_a_release_write_before() {
    let a = code(&format!("{MMIO}/access.rs"));
    order(&a, &["fn read_acquire", "read_volatile(ptr)", "fence i, iorw", "value"]);
    order(&a, &["fn write_release", "fence iorw, o", "write_volatile(ptr, value)"]);
}

#[test]
fn a_dma_sync_is_a_full_fence_either_way() {
    let d = code("arch/riscv64/dma_sync.rs");
    order(
        &d,
        &[
            "fn sync_for_device",
            "full_fence();",
            "fn sync_for_cpu",
            "full_fence();",
            "fn full_fence()",
            "fence iorw, iorw",
        ],
    );
}

#[test]
fn every_arch_has_a_backend() {
    for f in ["arch/mmio_order.rs", "arch/dma_sync.rs"] {
        let b = code(f);
        for arch in ["x86_64", "aarch64", "riscv64"] {
            assert!(b.contains(&format!("#[cfg(target_arch = \"{arch}\")]")), "{f}: {arch}");
        }
    }
}
