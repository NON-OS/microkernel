// NONOS Operating System (AGPL-3.0-or-later)
//! The Context's save and restore agree with its fields, and the riscv64
//! entry reaches the shared kernel, which hands riscv64 its boot memory.

use crate::source::{code, order};

const KERNEL: &str = "arch/riscv64/context/kernel.rs";

/// (register, offset) for each `op reg, off(a0)` in `asm`.
fn slots(asm: &str, op: &str) -> Vec<(String, u64)> {
    let pat = format!("\"{op} ");
    asm.split(&pat)
        .skip(1)
        .map(|rest| {
            let (reg, rest) = rest.split_once(", ").unwrap();
            (reg.to_string(), rest.split_once('(').unwrap().0.parse().unwrap())
        })
        .collect()
}

#[test]
fn save_and_restore_cover_the_fields_at_their_offsets() {
    let src = code(KERNEL);
    let want: Vec<(String, u64)> = (0..12)
        .map(|i| (format!("s{i}"), i * 8))
        .chain([("ra".to_string(), 96), ("sp".to_string(), 104)])
        .collect();
    assert_eq!(slots(&src, "sd"), want, "save_to");
    assert_eq!(slots(&src, "ld"), want, "restore_from");
    order(&src, &["pub s: [u64; 12],", "pub ra: u64,", "pub sp: u64,"]);
}

#[test]
fn the_entry_takes_the_shared_path() {
    order(
        &code("arch/riscv64/boot/entry.rs"),
        &[
            "super::init(&info);",
            "init_bootstrap();",
            "KernelHandoff::from_riscv64(&info);",
            "microkernel_init(&handoff);",
            "microkernel_main()",
        ],
    );
}

#[test]
fn the_shared_kernel_hands_riscv64_its_boot_memory() {
    let m = code("kernel_core/init/entry/init_arch_memory_and_framebuffer.rs");
    order(
        &m,
        &[
            "ArchSpecificHandoff::Riscv64 { info } => {",
            "crate::arch::riscv64::boot::init_boot_memory(info);",
        ],
    );
}
