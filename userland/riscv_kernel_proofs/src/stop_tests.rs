// NONOS Operating System (AGPL-3.0-or-later)
//! Where the riscv64 boot defers or stops: PMP left to the firmware, a console
//! that waits a bounded time, and a named stop while the VM layer is not
//! ported. Read from the kernel's source.

use crate::source::{code, order};

#[test]
fn pmp_is_left_to_the_firmware() {
    let init = code("arch/riscv64/security/init.rs");
    assert!(!init.contains("pmp"), "an S-mode kernel touches no PMP CSR");
    assert!(!std::path::Path::new("../../src/arch/riscv64/security/pmp.rs").exists());
}

#[test]
fn the_console_waits_a_bounded_time() {
    let c = code("arch/common/console_uart.rs");
    let rv = &c[c.find("mod rv_uart").expect("the riscv64 console")..];
    assert!(rv.contains("if (0..TX_TRIES).any(|_| port.is_tx_ready()) {\nport.putc(ch);\n}"));
}

#[test]
fn the_unported_vm_layer_stops_the_boot_by_name() {
    let p = code("arch/paging/ported.rs");
    order(&p, &["#[cfg(target_arch = \"riscv64\")]", "the VM layer is not ported", "halt_loop()"]);
    let vm = code("kernel_core/init/entry/init_vm_and_protection.rs");
    order(&vm, &["require_vm_port();", "init_unified_vm()"]);
}

#[test]
fn a_device_that_does_not_start_stops_the_boot_by_name() {
    let init = code("arch/riscv64/boot/init.rs");
    assert!(!init.contains("cpu::halt()"), "no silent halt in the boot steps");
    order(
        &init,
        &[
            "plic::init_plic(boot_info.plic_base).is_err()",
            "the PLIC did not start; stopping",
            "timer::init_timer().is_err()",
            "the timer did not start; stopping",
            "fn stop(why: &[u8]) -> ! {",
            "serial::println(why);",
            "halt_loop()",
        ],
    );
}
