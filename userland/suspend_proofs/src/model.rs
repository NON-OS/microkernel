// NONOS Operating System (AGPL-3.0-or-later)
//! The save and the resume run against a context, for one set of FLAGS: each
//! slot remembers which register it holds, and a read of a slot the save
//! never wrote, or into a register other than the one saved there, is the
//! failure being looked for.

use std::collections::{BTreeMap, BTreeSet};

use crate::header::value;
use crate::ops::Op;

/// What a run saved and put back, by register name.
pub struct Run {
    pub saved: BTreeSet<String>,
    pub restored: BTreeSet<String>,
}

fn on(cond: Option<i64>, flags: i64) -> bool {
    cond.is_none_or(|bit| flags >> bit & 1 == 1)
}

pub fn run(save: &[Op], restore: &[Op], flags: i64) -> Result<Run, String> {
    // MAGIC, MPIDR and FLAGS are Rust's, armed before the call.
    let mut slots: BTreeMap<i64, String> = ["CTX_MAGIC", "CTX_MPIDR", "CTX_FLAGS"]
        .iter()
        .map(|n| (value(n), format!("ctx@{}", value(n))))
        .collect();
    let mut saved = BTreeSet::new();
    for op in save {
        if let Op::Write { loc, off, cond } = op {
            if !on(*cond, flags) {
                continue;
            }
            if let Some(held) = slots.get(off).filter(|h| !h.starts_with("ctx@")) {
                return Err(format!("{loc} saved over {held} at {off}"));
            }
            if !(0..value("CTX_SIZE")).contains(off) {
                return Err(format!("{loc} saved outside the context, at {off}"));
            }
            slots.insert(*off, loc.clone());
            if !loc.starts_with("ctx@") {
                saved.insert(loc.clone());
            }
        }
    }
    let mut restored = BTreeSet::new();
    for op in restore {
        match op {
            Op::Read { loc, off, cond } if on(*cond, flags) => {
                let held = slots.get(off).ok_or(format!("{loc:?} read from unsaved {off}"))?;
                if let Some(loc) = loc {
                    if held != loc {
                        return Err(format!("{loc} put back from {held}'s slot"));
                    }
                    restored.insert(loc.clone());
                }
            }
            Op::Write { off, .. } if *off != value("CTX_MAGIC") => {
                return Err(format!("the resume writes the context at {off}"));
            }
            _ => {}
        }
    }
    Ok(Run { saved, restored })
}
