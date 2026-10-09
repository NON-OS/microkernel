/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/*
 * The kernel keeps its own copy of the app bits, since it links no userland
 * crate. Setup writes these bits and the kernel withholds spawns by its
 * copy, so a bit that meant one app here and another there would turn off
 * the wrong one. This reads the kernel's copy and holds it to these.
 */

use nonos_policy_proto::apps::{BROWSER, CALCULATOR, EDITOR, FILES, LINUX, MEDIA, STORE, WALLET};

const KERNEL: &str = include_str!("../../../src/userspace/init/app_choice/bits.rs");

fn kernel_bit(name: &str) -> u8 {
    let line = KERNEL
        .lines()
        .find(|l| l.starts_with(&format!("pub(crate) const {name}: u32 = 1 << ")))
        .unwrap_or_else(|| panic!("the kernel has no {name}"));
    let shift = line.rsplit("<< ").next().unwrap().trim_end_matches(';');
    1u8 << shift.parse::<u8>().unwrap()
}

#[test]
fn the_kernel_copy_of_the_app_bits_agrees() {
    let ours = [
        ("BROWSER", BROWSER),
        ("WALLET", WALLET),
        ("STORE", STORE),
        ("FILES", FILES),
        ("EDITOR", EDITOR),
        ("CALCULATOR", CALCULATOR),
        ("MEDIA", MEDIA),
        ("LINUX", LINUX),
    ];
    for (name, bit) in ours {
        assert_eq!(kernel_bit(name), bit, "{name}");
    }
    assert_eq!(KERNEL.matches(": u32 = 1 << ").count(), ours.len(), "no bit the kernel alone has");
}
