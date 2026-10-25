// NONOS Operating System (AGPL-3.0-or-later)
//! No key register is read or written before the per-CPU block's on word
//! says keys are switched: a CPU without pointer authentication has no such
//! registers, and TPIDR_EL1 being set says nothing about that any more.

const FILES: [(&str, &str); 3] = [
    ("vectors.S", include_str!("../../../src/arch/aarch64/asm/vectors.S")),
    ("enter_user.S", include_str!("../../../src/arch/aarch64/asm/enter_user.S")),
    ("resume_user.S", include_str!("../../../src/arch/aarch64/asm/resume_user.S")),
];
const SLOT: &str = include_str!("../../../src/arch/aarch64/security/pac/slot.rs");

/// Instruction lines, comments gone.
fn code(text: &str) -> Vec<String> {
    let strip = |l: &str| l.split("/*").next().unwrap_or("").trim().to_string();
    text.lines()
        .filter(|l| !l.trim_start().starts_with('*'))
        .map(strip)
        .filter(|l| !l.is_empty())
        .collect()
}

#[test]
fn every_key_register_access_follows_the_on_word() {
    for (name, text) in FILES {
        let lines = code(text);
        let keys: Vec<usize> =
            (0..lines.len()).filter(|&i| lines[i].contains("S3_0_C2_C1_")).collect();
        assert!(!keys.is_empty(), "{name} has no key access to check");
        for &k in &keys {
            // Walking up from the access, a gate comes before any label or
            // macro head: nothing can jump in past it.
            let gated = lines[..k].iter().rev().find_map(|l| {
                let gate = l.starts_with("PAC_ON") || l.contains("#PERCPU_PAC_ON]");
                let edge = l.ends_with(':') || l.starts_with(".macro");
                (gate || edge).then_some(gate)
            });
            assert_eq!(gated, Some(true), "{name}: {}", lines[k]);
        }
    }
    let gate = code(FILES[0].1);
    let at = gate.iter().position(|l| l == ".macro PAC_ON skip").expect("PAC_ON");
    assert_eq!(gate[at + 3], "ldr     x10, [x9, #PERCPU_PAC_ON]");
    assert_eq!(gate[at + 4], "cbz     x10, \\skip");
}

#[test]
fn keys_are_published_before_the_on_word_and_tpidr_is_left_alone() {
    let key = SLOT.find("block.pac[KERNEL..KERNEL + 2]").expect("the key store");
    let on = SLOT.find("block.pac[ON].store(1, Ordering::Release)").expect("the on store");
    assert!(key < on);
    assert!(!SLOT.contains("msr tpidr_el1"));
}
