// NONOS Operating System (AGPL-3.0-or-later)
//! Powerdown states only after the boot check passed: the check covers the
//! stub from its first instruction to the label after its last, and the
//! rest of the resume while the kernel runs one to one, and nothing but a
//! full pass turns powerdown on (the walk itself: boot_tables_proofs).

use crate::asm::lines;

const MAP: &str = include_str!("../../../src/arch/aarch64/idle_states/resume_map.rs");
const POWERDOWN: &str = include_str!("../../../src/arch/aarch64/idle_states/powerdown.rs");
const PREPARE: &str = include_str!("../../../src/arch/aarch64/idle_states/prepare.rs");

fn code(src: &str) -> String {
    let lines = src.lines().map(|l| l.split("//").next().unwrap_or("").trim());
    lines.filter(|l| !l.is_empty()).collect::<Vec<_>>().join("\n")
}

#[test]
fn the_end_labels_close_what_they_name() {
    let stub = lines("_aarch64_resume_start:", ".size _aarch64_resume_start");
    assert_eq!(stub.last().unwrap(), "br x16", "the stub's last instruction");
    let mut after = lines(".size _aarch64_resume_start", ".section .text.aarch64_resume_high");
    after.retain(|l| !l.starts_with('*'));
    assert_eq!(after, vec![".globl _aarch64_resume_start_end", "_aarch64_resume_start_end:"]);
    let tail: Vec<String> = lines(".size aarch64_resume_high", "aarch64_resume_high_end:");
    assert_eq!(tail, vec![".globl aarch64_resume_high_end"]);
}

#[test]
fn only_a_full_pass_turns_powerdown_on() {
    let map = code(MAP);
    assert_eq!(map.matches("VERIFIED.store(").count(), 1);
    let check = map.find("executable_one_to_one(root, start, end, reach)?;").unwrap();
    let spans = map.find("span(_aarch64_resume_start, _aarch64_resume_start_end)").unwrap();
    let high = map.find("if !runs_high() {").unwrap();
    let store = map.find("VERIFIED.store(true, Ordering::Release);").unwrap();
    assert!(spans < high && high < check && check < store);
    assert!(map.contains("published_root().ok_or("));
    assert!(code(POWERDOWN).contains("!resume_lost() && resume_map::verified()"));
    // Checked once, at boot, before any CPU is told what it may enter.
    let prepare = code(PREPARE);
    assert!(
        prepare.find("resume_map::verify()").unwrap() < prepare.find("allow(usize::MAX)").unwrap()
    );
}
