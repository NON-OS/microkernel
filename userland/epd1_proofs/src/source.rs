// NONOS Operating System (AGPL-3.0-or-later)
//! The kernel sources the text proofs read, compiled in so a change to any
//! of them reruns the proofs.

pub const HEADER: &str = include_str!("../../../src/arch/aarch64/asm/epd1.h");
pub const TRAMPOLINE: &str = include_str!("../../../src/arch/aarch64/asm/epd1_trampoline.S");
pub const VECTORS: &str = include_str!("../../../src/arch/aarch64/asm/vectors.S");
pub const ENTER_USER: &str = include_str!("../../../src/arch/aarch64/asm/enter_user.S");
pub const RESUME_USER: &str = include_str!("../../../src/arch/aarch64/asm/resume_user.S");
pub const SPECTRE_ENTRY: &str = include_str!("../../../src/arch/aarch64/security/spectre/entry.rs");
pub const SPECTRE_WORK: &str = include_str!("../../../src/arch/aarch64/asm/spectre_work.h");

/// Every file of the kernel's layout constants.
pub const LAYOUT: [&str; 12] = [
    include_str!("../../../src/memory/layout/constants/canonical.rs"),
    include_str!("../../../src/memory/layout/constants/firmware.rs"),
    include_str!("../../../src/memory/layout/constants/fixmap.rs"),
    include_str!("../../../src/memory/layout/constants/kpti.rs"),
    include_str!("../../../src/memory/layout/constants/limits.rs"),
    include_str!("../../../src/memory/layout/constants/mmio.rs"),
    include_str!("../../../src/memory/layout/constants/page.rs"),
    include_str!("../../../src/memory/layout/constants/percpu.rs"),
    include_str!("../../../src/memory/layout/constants/permissions.rs"),
    include_str!("../../../src/memory/layout/constants/pt_index.rs"),
    include_str!("../../../src/memory/layout/constants/regions.rs"),
    include_str!("../../../src/memory/layout/constants/sections.rs"),
];

/// A `#define NAME value` from the header, as a number.
pub fn define(name: &str) -> u64 {
    let line = HEADER
        .lines()
        .find(|l| l.split_whitespace().nth(1) == Some(name) && l.starts_with("#define"))
        .unwrap_or_else(|| panic!("{name} is not defined in epd1.h"));
    let value: String = line.split_whitespace().skip(2).collect();
    let shift = value.strip_prefix("(1<<").and_then(|v| v.strip_suffix(')'));
    let parsed = match shift {
        Some(n) => n.parse::<u32>().ok().map(|n| 1u64 << n),
        None => number(&value),
    };
    parsed.unwrap_or_else(|| panic!("{name}: {value} is not a number"))
}

/// A hexadecimal or decimal literal, underscores allowed.
pub fn number(text: &str) -> Option<u64> {
    let text = text.replace('_', "");
    match text.strip_prefix("0x").or_else(|| text.strip_prefix("0X")) {
        Some(hex) => u64::from_str_radix(hex, 16).ok(),
        None => text.parse().ok(),
    }
}

/// The lines of `text` after the one equal to `start`, up to the next
/// label (in column 0, ending in `:`), `.size` or `.endm`.
pub fn block<'a>(text: &'a str, start: &str) -> Vec<&'a str> {
    let mut lines = text.lines().skip_while(|l| l.trim() != start);
    assert!(lines.next().is_some(), "{start} not found");
    let end = |l: &str| {
        let t = l.trim();
        (l.ends_with(':') && !l.starts_with(char::is_whitespace))
            || t.starts_with(".size")
            || t == ".endm"
    };
    lines.take_while(|l| !end(l)).collect()
}
