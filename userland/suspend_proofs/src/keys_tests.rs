// NONOS Operating System (AGPL-3.0-or-later)
//! No key in the context: the save never reads a key register, nothing the
//! context is written from is one, the resume loads keys only from the
//! per-CPU block TPIDR_EL1 names, and the context is a kernel static.

use crate::asm::lines;

const CONTEXT_RS: &str = include_str!("../../../src/arch/aarch64/idle_states/context.rs");
const KEY_REGS: [&str; 5] = ["s3_0_c2_c1_", "s3_0_c2_c2_", "s3_0_c2_c3_", "apia", "apib"];

fn is_key(line: &str) -> bool {
    KEY_REGS.iter().any(|k| line.to_lowercase().contains(k))
}

#[test]
fn the_save_never_touches_a_key() {
    let save = lines("aarch64_cpu_suspend:", ".size aarch64_cpu_suspend");
    assert!(save.iter().all(|l| !is_key(l)), "a key register in the save");
}

#[test]
fn the_resume_takes_keys_from_the_per_cpu_block_only() {
    let resume = lines("aarch64_resume_high:", ".L_resume_lost:");
    let keys: Vec<usize> = (0..resume.len()).filter(|&i| is_key(&resume[i])).collect();
    assert_eq!(keys.len(), 2, "{resume:?}");
    assert!(keys.iter().all(|&i| resume[i].starts_with("msr ")));
    let load = resume.iter().position(|l| l == "ldp x11, x12, [x9, #PERCPU_PAC_KERNEL]").unwrap();
    let block = resume.iter().position(|l| l == "mrs x9, tpidr_el1").unwrap();
    let on = resume.iter().position(|l| l == "ldr x11, [x9, #PERCPU_PAC_ON]").unwrap();
    assert!(block < on && on < load && load < keys[0]);
    assert!(resume[on + 1].starts_with("cbz x11,") && resume[block + 1].starts_with("cbz x9,"));
    for line in &resume[load + 1..keys[1]] {
        assert!(!line.contains("x11") && !line.contains("x12") || is_key(line), "{line}");
    }
    // Neither key register is stored anywhere, in the context or out of it.
    let all = lines(".section .text.aarch64_cpu_suspend", ".size aarch64_resume_high");
    assert!(all.iter().all(|l| !(l.starts_with("st") && (l.contains("x11") || l.contains("x12")))));
}

#[test]
fn the_context_is_a_kernel_static_with_no_key_field() {
    let code: String = CONTEXT_RS.lines().filter(|l| !l.trim_start().starts_with("//")).collect();
    assert!(code.contains("pub(super) static SLOTS: Slots"));
    assert!(code.contains("words: [u64; WORDS],") && code.contains("q: [u128; 32],"));
    assert!(!code.to_lowercase().contains("key"));
    assert!(!code.contains("alloc") && !code.contains("Box"));
}
