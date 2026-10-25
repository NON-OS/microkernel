// NONOS Operating System (AGPL-3.0-or-later)
//! `suspend.S` cut into its paths: the save, the resume, and the return of
//! a call that stayed up.

use crate::header::SUSPEND_S;
use crate::ops::{ops, Op};

/// The lines from the one starting `from` to the one starting `to`.
pub fn lines(from: &str, to: &str) -> Vec<String> {
    let all: Vec<String> = SUSPEND_S.lines().map(clean).collect();
    let start = all.iter().position(|l| l.starts_with(from)).expect(from) + 1;
    let len = all[start..].iter().position(|l| l.starts_with(to)).expect(to);
    all[start..start + len].iter().filter(|l| !l.is_empty()).cloned().collect()
}

fn clean(line: &str) -> String {
    let line = line.split("/*").next().unwrap_or("");
    line.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// The save, the resume, and the return of a call that stayed up.
pub fn save() -> Vec<Op> {
    ops(&lines("aarch64_cpu_suspend:", "mov x19, x0"))
}

pub fn restore() -> Vec<Op> {
    ops(&lines("aarch64_resume_high:", ".L_resume_lost:"))
}

pub fn stayed_up() -> Vec<Op> {
    ops(&lines("mov x0, x19", "ret"))
}
