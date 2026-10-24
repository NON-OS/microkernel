// NONOS Operating System (AGPL-3.0-or-later)
//! The copy installed through the shared checks only where the kernel stays,
//! and its word on what was verified the only one the measurement takes.

use crate::source::{code, order, raw};

#[test]
fn installed_through_the_shared_checks_where_the_kernel_stays() {
    let finish = code("arch/aarch64/boot/finish.rs");
    order(
        &finish,
        &[
            "pub(super) fn finish(info: &BootInfo) -> ! {",
            "if let Err(e) = super::loader::install() {",
            "super::refuse(b\"loader handoff\"",
            "super::core_entry::enter(info)",
        ],
    );
    let install = code("arch/aarch64/boot/loader/install.rs");
    order(
        &install,
        &[
            "let Some(copy) = COPY.get() else {",
            "return Ok(None);",
            "init_handoff(copy as *const BootHandoffV1 as u64)?",
        ],
    );
    let from = code("boot/handoff/kernel_handoff/aarch64/from.rs");
    order(
        &from,
        &[
            "let loader = crate::boot::handoff::get_handoff();",
            "measurement: builders::measurement(loader),",
        ],
    );
    let builders = code("boot/handoff/kernel_handoff/aarch64/builders.rs");
    assert!(builders.contains("secure_boot: loader.is_some_and(|h| h.secure_boot_enabled()),"));
    assert!(builders
        .contains("kernel_signature_verified: loader.is_some_and(|h| h.kernel_verified()),"));
    let fb = code("kernel_core/init/entry/init_arch_framebuffer.rs");
    order(
        &fb,
        &[
            "ArchSpecificHandoff::Aarch64 { .. } => {",
            "get_handoff().and_then(|h| h.framebuffer().copied());",
            "loader.or_else(crate::arch::aarch64::firmware::framebuffer)",
        ],
    );
    let ld = raw("linker_aarch64.ld");
    assert!(ld.contains("__kernel_start = __kernel_image_start;"));
    assert!(ld.contains("__kernel_end = __kernel_image_end;"));
}
