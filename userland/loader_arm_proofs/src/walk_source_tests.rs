// NONOS Operating System (AGPL-3.0-or-later)
//! What the walk may do with the data cache off, and that the loader builds
//! it over the real system registers.

use crate::source::{code, order};

const ENTER: &str = "arch/aarch64/asm/enter_kernel.S";

#[test]
fn no_load_or_store_once_the_data_cache_is_off() {
    let src = code(ENTER);
    let after = &src[src.find("sctlr_clear 0x4").unwrap()..];
    let walk = code("arch/aarch64/asm/walk.S");
    for text in [after, walk.as_str()] {
        // Every A64 load and store mnemonic starts ld or st: ldr, ldrb, ldxr,
        // ldnp, stlr, st1 and the rest.
        let access = text.split_whitespace().find(|w| {
            (w.starts_with("ld") || w.starts_with("st"))
                && w.chars().all(|c| c.is_ascii_alphanumeric())
        });
        assert_eq!(access, None, "a load or store with the cache off");
    }
}

#[test]
fn the_loader_assembles_the_real_registers_before_the_walk() {
    let m = code("arch/aarch64/asm/embed.rs");
    order(
        &m,
        &[
            "include_str!(\"sysregs.S\")",
            "include_str!(\"walk.S\")",
            "include_str!(\"enter_kernel.S\")",
        ],
    );
    order(
        &code("arch/aarch64/asm/sysregs.S"),
        &[
            "mrs     \\reg, clidr_el1",
            "mrs     \\reg, id_aa64mmfr2_el1",
            "msr     csselr_el1, \\sel",
            "isb",
            "mrs     \\out, ccsidr_el1",
            "dc      cisw, \\reg",
        ],
    );
}

#[test]
fn each_level_ends_in_a_barrier_before_the_next() {
    order(
        &code("arch/aarch64/asm/walk.S"),
        &["b.hs    94b", "dsb     sy", "96: add     x5, x5, #1", "b.lo    90b", "97: dsb     sy"],
    );
}
