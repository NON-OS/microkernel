// NONOS Operating System (AGPL-3.0-or-later)
//! F-97: one seeded hostile table and what each of its methods must give
//! under LIMITS: a long loop (Steps), recursion of a random depth
//! (CallDepth past the limit, else Zero), a buffer of a random size (Alloc
//! past the budget), a nested package with lying counts, and byte soup.

use super::hostile_gen::*;
use crate::enc::*;
use crate::exec::{ExecLimits, FaultKind};

pub const LIMITS: ExecLimits = ExecLimits {
    steps: 5_000,
    call_depth: 16,
    stack_bytes: 256 << 10,
    alloc: 64 << 10,
    time_ms: 50,
};

/// One table: its body, and what calling each method with Arg0 must give.
pub fn hostile(s: &mut u64, i: usize) -> (Vec<u8>, Vec<(String, u64, Want)>) {
    let (l, r, b, x) =
        (format!("L{i:03}"), format!("R{i:03}"), format!("B{i:03}"), format!("S{i:03}"));
    let depth = next(s) % 40;
    let size = next(s) % (2 * LIMITS.alloc as u64);
    let nest = (next(s) % 12) as u32;
    let body = cat(&[
        &name_obj(&format!("P{i:03}"), &nested_pkg(s, nest)),
        &method(&format!("\\{l}"), 0, &long_loop(100_000)),
        &method(&format!("\\{r}"), 1, &recursion(&r)),
        &method(&format!("\\{b}"), 0, &big_buffer(size)),
        &method(&format!("\\{x}"), 1, &soup(s, 64)),
    ]);
    let deep = if depth >= u64::from(LIMITS.call_depth) {
        Want::Fault(FaultKind::CallDepth)
    } else {
        Want::Value(0)
    };
    let alloc = if size > LIMITS.alloc as u64 { Want::Fault(FaultKind::Alloc) } else { Want::Any };
    let wants = vec![
        (l, 0, Want::Fault(FaultKind::Steps)),
        (r, depth, deep),
        (b, 0, alloc),
        (x, next(s) & 0xFF, Want::Any),
    ];
    (body, wants)
}
