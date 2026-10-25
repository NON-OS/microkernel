// NONOS Operating System (AGPL-3.0-or-later)
//! The `#define`s of `asm/suspend.h` and `asm/percpu.h`, by name.

use std::collections::BTreeMap;

pub const SUSPEND_H: &str = include_str!("../../../src/arch/aarch64/asm/suspend.h");
pub const PERCPU_H: &str = include_str!("../../../src/arch/aarch64/asm/percpu.h");
pub const SUSPEND_S: &str = include_str!("../../../src/arch/aarch64/asm/suspend.S");

pub fn defines() -> BTreeMap<String, i64> {
    let mut out = BTreeMap::new();
    for line in SUSPEND_H.lines().chain(PERCPU_H.lines()) {
        let mut words = line.split_whitespace();
        if words.next() != Some("#define") {
            continue;
        }
        let (Some(name), Some(value)) = (words.next(), words.next()) else { continue };
        let value = match value.strip_prefix("0x") {
            Some(hex) => i64::from_str_radix(hex, 16),
            None => value.parse(),
        };
        out.insert(name.to_string(), value.unwrap_or_else(|_| panic!("{name} unreadable")));
    }
    out
}

pub fn value(name: &str) -> i64 {
    *defines().get(name).unwrap_or_else(|| panic!("no #define {name}"))
}
