// NONOS Operating System (AGPL-3.0-or-later)
//! `suspend.h` against itself and against the Rust side (`layout.rs`): one
//! word per slot, inside the context, Q0-Q31 16-aligned at the end, and the
//! values Rust writes and tests the same on both sides.

use crate::header::{defines, value, SUSPEND_H};
use crate::layout;

#[test]
fn the_slots_are_distinct_words_before_the_vector_registers() {
    let words: Vec<(String, i64)> = defines()
        .into_iter()
        .filter(|(n, _)| n.starts_with("CTX_"))
        .filter(|(n, _)| !["CTX_Q", "CTX_SIZE", "CTX_ARMED"].contains(&n.as_str()))
        .collect();
    let mut offsets: Vec<i64> = words.iter().map(|(_, o)| *o).collect();
    offsets.sort();
    offsets.dedup();
    assert_eq!(offsets.len(), words.len(), "two slots share a word");
    assert!(offsets.iter().all(|o| o % 8 == 0 && *o < value("CTX_Q")));
    assert_eq!(value("CTX_Q") % 16, 0);
    assert_eq!(value("CTX_Q") + 32 * 16, value("CTX_SIZE"));
}

#[test]
fn rust_and_the_header_agree() {
    assert_eq!(layout::MAGIC as i64 * 8, value("CTX_MAGIC"));
    assert_eq!(layout::MPIDR as i64 * 8, value("CTX_MPIDR"));
    assert_eq!(layout::FLAGS as i64 * 8, value("CTX_FLAGS"));
    assert_eq!(layout::WORDS as i64 * 8, value("CTX_Q"));
    assert_eq!(layout::Q_OFFSET as i64, value("CTX_Q"));
    assert_eq!(layout::SIZE as i64, value("CTX_SIZE"));
    assert_eq!(layout::ARMED as i64, value("CTX_ARMED"));
    assert_eq!(layout::RESUMED, value("SUSPEND_RESUMED"));
    let flags = [
        (layout::FLAG_GICV3, "FLAG_GICV3"),
        (layout::FLAG_PAN, "FLAG_PAN"),
        (layout::FLAG_SVE, "FLAG_SVE"),
        (layout::FLAG_MTE2, "FLAG_MTE2"),
        (layout::FLAG_SSBS, "FLAG_SSBS"),
    ];
    for (bit, name) in flags {
        assert_eq!(bit, 1 << value(name), "{name}");
        assert_ne!(value(name), value("FLAG_FP"), "{name} is the save's own bit");
    }
}

#[test]
fn the_armed_word_is_built_as_defined() {
    let body = SUSPEND_H.split(".macro ARMED_INTO").nth(1).unwrap().split(".endm").next().unwrap();
    let mut built = 0u64;
    for line in body.lines().map(str::trim).filter(|l| l.starts_with("mov")) {
        let imm = line.split('#').nth(1).unwrap().split(',').next().unwrap().trim();
        let imm = u64::from_str_radix(imm.trim_start_matches("0x"), 16).unwrap();
        let shift = line.split("lsl #").nth(1).map_or(0, |s| s.trim().parse::<u32>().unwrap());
        built = if line.starts_with("movz") { imm << shift } else { built | imm << shift };
    }
    assert_eq!(built, value("CTX_ARMED") as u64);
    assert_eq!(built, layout::ARMED);
}
