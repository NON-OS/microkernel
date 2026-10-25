// NONOS Operating System (AGPL-3.0-or-later)
//! The std platform layer reaches the kernel through one file per concern,
//! with each architecture's registers matching what the kernel reads:
//! nothing else in the layer names a register or an instruction.

use std::fs;
use std::path::Path;

pub(crate) const STD: &str = "../../toolchain/nonos-std";
pub(crate) const OWNERS: [&str; 2] = ["sys/pal/nonos/trap.rs", "sys/pal/nonos/thread_regs.rs"];

pub(crate) fn read(rel: &str) -> String {
    fs::read_to_string(format!("{STD}/{rel}")).unwrap()
}

fn rust_files(dir: &Path, out: &mut Vec<String>) {
    for e in fs::read_dir(dir).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() {
            rust_files(&p, out);
        } else if p.extension().is_some_and(|x| x == "rs") {
            out.push(p.to_string_lossy().into_owned());
        }
    }
}

#[test]
fn only_the_two_owners_hold_assembly() {
    let mut files = Vec::new();
    rust_files(Path::new(STD), &mut files);
    assert!(files.len() > 20, "the layer was not read: {}", files.len());
    for f in files {
        let owner = OWNERS.iter().any(|o| f.ends_with(o));
        let asm = fs::read_to_string(&f).unwrap().contains("asm!");
        assert!(owner || !asm, "{f} holds assembly: call pal::trap or pal::thread_regs");
    }
}

#[test]
fn each_arm_uses_the_registers_the_kernel_reads() {
    let trap = read(OWNERS[0]);
    let x86 = ["inout(\"rax\") num => r", "in(\"r10\") a4", "\"syscall\""];
    let a64 = ["\"svc #0\"", "inlateout(\"x0\") a1 => r", "in(\"x8\") num"];
    let rv = ["\"ecall\"", "inlateout(\"a0\") a1 => r", "in(\"a7\") num"];
    for (arch, regs) in [("x86_64", x86), ("aarch64", a64), ("riscv64", rv)] {
        let arm = arm_of(&trap, arch);
        for reg in regs {
            assert!(arm.contains(reg), "{arch}: {reg} not in its own arm");
        }
    }
    let svc = fs::read_to_string("../../src/arch/aarch64/exceptions/handlers/svc.rs").unwrap();
    assert!(svc.contains("from_u64(frame.x8)") && svc.contains("frame.x0 = result_word"));
    let ecall = "../../src/arch/riscv64/interrupts/handlers/syscall.rs";
    let ecall = fs::read_to_string(ecall).unwrap();
    assert!(ecall.contains("from_u64(frame.a7 as u64)") && ecall.contains("frame.a0 ="));
}

/// The text under `#[cfg(target_arch = "<arch>")]`, up to the next such gate.
fn arm_of<'a>(text: &'a str, arch: &str) -> &'a str {
    let gate = format!("#[cfg(target_arch = \"{arch}\")]");
    let from = text.find(&gate).unwrap_or_else(|| panic!("no {arch} arm")) + gate.len();
    let rest = &text[from..];
    &rest[..rest.find("#[cfg(").unwrap_or(rest.len())]
}
