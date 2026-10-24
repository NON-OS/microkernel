// NONOS Operating System (AGPL-3.0-or-later)
//! Over every combination of optional parts: each register saved is put back
//! from its own slot, nothing is put back that was not saved, and a call
//! that stayed up hands back exactly the callee-saved registers.

use crate::asm::{restore, save, stayed_up};
use crate::header::value;
use crate::model::run;

fn all_flags() -> impl Iterator<Item = i64> {
    0..1 << (value("FLAG_SSBS") + 1)
}

#[test]
fn every_saved_register_comes_back_and_nothing_else() {
    let (save, restore) = (save(), restore());
    for flags in all_flags() {
        let run = run(&save, &restore, flags).unwrap_or_else(|e| panic!("flags {flags:#x}: {e}"));
        assert_eq!(run.saved, run.restored, "flags {flags:#x}");
    }
}

#[test]
fn the_parts_follow_their_flags() {
    let (save, restore) = (save(), restore());
    let base = run(&save, &restore, 0).unwrap().saved;
    for reg in ["sctlr_el1", "tcr_el1", "mair_el1", "ttbr0_el1", "ttbr1_el1", "vbar_el1"]
        .into_iter()
        .chain(["tpidr_el1", "tpidr_el0", "tpidrro_el0", "sp_el0", "cpacr_el1", "cntkctl_el1"])
        .chain(["cntp_ctl_el0", "cntp_cval_el0", "cntv_ctl_el0", "cntv_cval_el0", "mdscr_el1"])
        .chain(["oslsr_el1", "sp", "x18", "x19", "x29", "x30"])
    {
        assert!(base.contains(reg), "{reg} not saved");
    }
    let parts = [
        ("FLAG_FP", vec!["fpcr", "fpsr", "q0", "q17", "q31"]),
        ("FLAG_PAN", vec!["s3_0_c4_c2_3"]),
        ("FLAG_SVE", vec!["s3_0_c1_c2_0"]),
        ("FLAG_MTE2", vec!["s3_0_c1_c6_6"]),
        ("FLAG_GICV3", vec!["s3_0_c12_c12_5", "s3_0_c4_c6_0", "s3_0_c12_c12_7"]),
    ];
    for (flag, regs) in parts {
        let with = run(&save, &restore, 1 << value(flag)).unwrap().saved;
        for reg in regs {
            assert!(!base.contains(reg) && with.contains(reg), "{reg} not under {flag}");
        }
    }
    let fp = run(&save, &restore, 1 << value("FLAG_FP")).unwrap().saved;
    assert_eq!(fp.iter().filter(|r| r.starts_with('q')).count(), 32);
}

#[test]
fn a_call_that_stayed_up_returns_the_callee_saved_registers() {
    let saved = run(&save(), &[], 0).unwrap().saved;
    let back = run(&save(), &stayed_up(), 0).unwrap().restored;
    let callee: Vec<String> = (18..=30).map(|n| format!("x{n}")).collect();
    assert_eq!(back.iter().cloned().collect::<Vec<_>>().len(), callee.len());
    assert!(callee.iter().all(|r| back.contains(r) && saved.contains(r)));
}
