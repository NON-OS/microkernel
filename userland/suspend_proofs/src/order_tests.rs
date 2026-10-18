// NONOS Operating System (AGPL-3.0-or-later)
//! The order the path keeps: the context written out before the call, the
//! context checked before anything is put back, translation first, the key
//! before EnIA, FP access before FP registers, a lost resume shutting every
//! CPU out of powerdown states, and Rust arming and disarming the context.

use crate::asm::lines;

const POWERDOWN: &str = include_str!("../../../src/arch/aarch64/idle_states/powerdown.rs");
const CONTEXT: &str = include_str!("../../../src/arch/aarch64/idle_states/context.rs");

fn at(body: &[String], line: &str) -> usize {
    body.iter()
        .position(|l| l.trim_start_matches(|c: char| c.is_ascii_digit() || c == ':').trim() == line)
        .unwrap_or_else(|| panic!("no `{line}`"))
}

#[test]
fn the_context_reaches_memory_before_the_call() {
    let save = lines("aarch64_cpu_suspend:", ".size aarch64_cpu_suspend");
    let last_store = save.iter().rposition(|l| l.starts_with("stp q30")).unwrap();
    let clean = at(&save, "dc cvac, x9");
    let fence = at(&save, "dsb sy");
    let call = at(&save, "smc #0").min(at(&save, "hvc #0"));
    assert!(last_store < clean && clean < fence && fence < call);
    assert!(
        save.contains(&"movk w0, #0xc400, lsl #16".into())
            && save.contains(&"movz w0, #0x0001".into())
    );
}

#[test]
fn the_resume_checks_then_restores_in_order() {
    let r = lines("aarch64_resume_high:", ".L_resume_lost:");
    let first_load = r.iter().position(|l| l.contains("LOAD")).unwrap();
    let lost: Vec<usize> = (0..r.len()).filter(|&i| r[i] == "b.ne .L_resume_lost").collect();
    assert_eq!(lost.len(), 2);
    assert!(lost.iter().all(|&i| i < first_load));
    assert!(r[lost[0] - 2].contains("CTX_MAGIC") && r[lost[1] - 2].contains("CTX_MPIDR"));
    let ttbr0 = at(&r, "LOAD ttbr0_el1, CTX_TTBR0");
    let tlbi = at(&r, "tlbi vmalle1");
    let tpidr = at(&r, "LOAD tpidr_el1, CTX_TPIDR_EL1");
    let key = at(&r, "msr S3_0_C2_C1_0, x11");
    let sctlr = at(&r, "LOAD sctlr_el1, CTX_SCTLR");
    assert!(ttbr0 < tlbi && tlbi < tpidr && tpidr < key && key < sctlr);
    let cpacr = at(&r, "LOAD cpacr_el1, CTX_CPACR");
    assert!(cpacr < at(&r, "ldp q0, q1, [x9, #0]") && r[cpacr + 1] == "isb");
    let sre = at(&r, "LOAD S3_0_C12_C12_5, CTX_ICC_SRE");
    assert!(r[sre + 1] == "isb" && sre < at(&r, "LOAD S3_0_C4_C6_0, CTX_ICC_PMR"));
    let disarm = at(&r, "str xzr, [x0, #CTX_MAGIC]");
    let last_load = r.iter().rposition(|l| l.contains("LOAD") || l.starts_with("ldp q")).unwrap();
    let ret = at(&r, "ret");
    assert!(last_load < disarm && disarm < ret && r[ret - 1] == "mov x0, #SUSPEND_RESUMED");
}

#[test]
fn a_lost_resume_shuts_out_powerdown_and_parks() {
    let lost = lines(".L_resume_lost:", ".size aarch64_resume_high");
    assert_eq!(lost[0], "adrp x9, AARCH64_RESUME_LOST");
    assert!(lost.contains(&"str x10, [x9]".into()) && lost.contains(&"mov x10, #1".into()));
    assert!(lost.iter().any(|l| l.ends_with("wfi")) && lost.last().unwrap().starts_with("b "));
    assert!(CONTEXT.contains("static AARCH64_RESUME_LOST: AtomicU64"));
    assert!(POWERDOWN.contains("conduit().is_some() && !resume_lost()"));
}

#[test]
fn rust_arms_the_context_before_the_call_and_disarms_it_after() {
    let code: Vec<&str> = POWERDOWN.lines().map(str::trim).collect();
    let arm = code.iter().position(|l| *l == "(*ctx).words[MAGIC] = ARMED;").unwrap();
    let call = code.iter().position(|l| l.contains("aarch64_cpu_suspend(ctx,")).unwrap();
    let disarm = code.iter().position(|l| l.contains("(*ctx).words[MAGIC] = 0")).unwrap();
    assert!(arm < call && call < disarm);
    assert!(code[arm + 1].contains("words[MPIDR] = mpidr_affinity()"));
    assert!(code.contains(&"RESUMED => Ok(true),"));
}
