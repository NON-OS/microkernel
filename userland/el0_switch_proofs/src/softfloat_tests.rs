// NONOS Operating System (AGPL-3.0-or-later)
//! The aarch64 kernel is built without FP/SIMD, Rust and C alike, and only
//! the routines that save and restore the registers may name them.

use crate::source::raw;

#[test]
fn rust_and_c_leave_the_vector_registers_alone() {
    let target = raw("aarch64-nonos.json");
    let features = target.lines().find(|l| l.contains("\"features\"")).unwrap();
    assert!(features.contains("-neon") && features.contains("-fp-armv8"), "{features}");
    assert!(target.contains("\"abi\": \"softfloat\""));
    let flags = raw("build_c_target.rs");
    assert!(flags.contains(
        "\"aarch64\" => Some((\"aarch64-unknown-none-elf\", &[\"-mgeneral-regs-only\"][..])),"
    ));
    // Every C compile, the assembly and both PQClean schemes, takes its flags
    // from there and applies all of them.
    let build = raw("build.rs");
    assert_eq!(build.matches("build_c_target::c_target(&arch)").count(), 3);
    assert_eq!(build.matches("for flag in arch_flags {\n        build.flag(flag);").count(), 1);
    assert_eq!(build.matches("for flag in c_flags {\n        build.flag(flag);").count(), 2);
}

#[test]
fn only_the_saving_routines_may_name_them() {
    let dir = "../../src/arch/aarch64/asm";
    let mut allowed: Vec<String> = std::fs::read_dir(dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| std::fs::read_to_string(p).unwrap().contains(".arch_extension fp"))
        .map(|p| p.file_name().unwrap().to_string_lossy().into_owned())
        .collect();
    allowed.sort();
    assert_eq!(allowed, ["fpu_restore.S", "fpu_save.S", "suspend.S"]);
    // The ELF check's owners are those files' routines, and no others.
    let check = raw("scripts/check_kernel_elf.py");
    for owner in ["aarch64_fpu_save", "aarch64_fpu_restore", "aarch64_cpu_suspend"] {
        assert!(check.contains(&format!("\"{owner}\"")), "{owner}");
    }
    assert!(check.contains("\"aarch64_resume_high\""));
    assert!(check.contains("failed += vector_registers(elf)"));
}
