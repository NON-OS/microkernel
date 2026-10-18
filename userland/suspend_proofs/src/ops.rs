// NONOS Operating System (AGPL-3.0-or-later)
//! A line of `suspend.S` as what it does to the context: a register saved
//! to an offset, or put back from one, and under which FLAGS bit.

use crate::header::value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Op {
    /// `loc` stored at `off` when FLAGS bit `cond` is set (or always).
    Write { loc: String, off: i64, cond: Option<i64> },
    /// `off` read, into `loc` when it puts a register back.
    Read { loc: Option<String>, off: i64, cond: Option<i64> },
}

pub fn ops(lines: &[String]) -> Vec<Op> {
    let (mut out, mut cond, mut until) = (Vec::new(), None, String::new());
    for line in lines {
        let (label, line) = match line.split_once(':') {
            Some((l, rest)) if l.parse::<u32>().is_ok() => (l, rest.trim()),
            _ => ("", line.as_str()),
        };
        if !label.is_empty() && label == until {
            cond = None;
        }
        let words: Vec<&str> =
            line.split([' ', ',', '[', ']', '#']).filter(|w| !w.is_empty()).collect();
        match words.as_slice() {
            ["tbz", "x10", flag, target] => {
                (cond, until) = (Some(value(flag)), target.trim_end_matches('f').into())
            }
            ["SAVE", reg, off] => {
                out.push(Op::Write { loc: reg.to_lowercase(), off: value(off), cond })
            }
            ["LOAD", reg, off] => {
                out.push(Op::Read { loc: Some(reg.to_lowercase()), off: value(off), cond })
            }
            [op @ ("stp" | "ldp"), a, b, base, off] => {
                // Q0-Q31 go through x9 = x0 + CTX_Q; x9 with a name is the
                // per-CPU block the keys are read from, not the context.
                let (off, step) = match (*base, off.parse::<i64>()) {
                    ("x0", _) => (value(off), 8),
                    ("x9", Ok(at)) => (value("CTX_Q") + at, 16),
                    _ => continue,
                };
                for (loc, off) in [(a, off), (b, off + step)] {
                    out.push(access(*op == "stp", Some(loc.to_string()), off, cond));
                }
            }
            [op @ ("str" | "ldr"), reg, "x0", off] => {
                let off = value(off);
                out.push(access(*op == "str", named(reg, off), off, cond));
            }
            _ => {}
        }
    }
    out
}

/// The register a single load or store moves. A scratch register's store
/// is to a word Rust owns: FLAGS, or MAGIC when the resume disarms it.
fn named(reg: &str, off: i64) -> Option<String> {
    match off {
        _ if off == value("CTX_SP") => Some("sp".into()),
        _ if off == value("CTX_OSLSR") => Some("oslsr_el1".into()),
        _ if ["x9", "x10", "xzr"].contains(&reg) => None,
        _ => Some(reg.into()),
    }
}

fn access(write: bool, loc: Option<String>, off: i64, cond: Option<i64>) -> Op {
    match write {
        true => Op::Write { loc: loc.unwrap_or(format!("ctx@{off}")), off, cond },
        false => Op::Read { loc, off, cond },
    }
}
