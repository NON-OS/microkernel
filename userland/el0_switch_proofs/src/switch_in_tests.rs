// NONOS Operating System (AGPL-3.0-or-later)
//! A task taken up by a CPU gets its own registers there, and one switched
//! away inside the kernel resumes there rather than at an older snapshot.

use crate::source::{code, order};

#[test]
fn a_kernel_context_wins_over_a_user_snapshot() {
    let dispatch = code("arch/aarch64/context/switch/dispatch.rs");
    order(
        &dispatch,
        &[
            "if try_first_entry(&pcb, pid) {",
            "if try_resume_kernel(&pcb, pid) {",
            "if try_resume(&pcb, pid) {",
        ],
    );
    let resume = code("arch/aarch64/context/switch/kernel_resume.rs");
    order(
        &resume,
        &[
            "INTERRUPT_SAVED_CONTEXTS.write().remove(&pid) else {",
            "return false;",
            "if pcb.kernel_stack_top.load(Ordering::Acquire) == 0 {",
            "if swap_address_space(pcb).is_err() {",
            "CURRENT_PID.store(pid, Ordering::SeqCst);",
            "fpu::prepare_incoming();",
            "write_user_tls(pcb.get_tls_base());",
            "crate::arch::aarch64::security::enter_process(",
            "ctx.restore()",
        ],
    );
}

#[test]
fn every_way_in_installs_its_own_thread_register() {
    for (file, enter) in
        [("first_entry.rs", "enter_user(&entry)"), ("resume.rs", "resume_user(&saved)")]
    {
        let src = code(&format!("arch/aarch64/context/switch/{file}"));
        order(&src, &["fpu::prepare_incoming();", "write_user_tls(pcb.get_tls_base());", enter]);
    }
    let tls = code("arch/context/tls.rs");
    order(&tls, &["#[cfg(target_arch = \"aarch64\")]", "write_user_tls(base);", "true"]);
    assert!(
        code("arch/aarch64/context/tls.rs").contains("asm!(\"msr tpidr_el0, {}\", in(reg) base")
    );
}
