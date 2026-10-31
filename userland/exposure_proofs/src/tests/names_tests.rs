// NONOS Operating System (AGPL-3.0-or-later)
//! Every named Admin holder is a name the kernel spawns, and every claim is
//! short enough for the About screen to show whole.

use crate::held_by::{ADMIN_CLAIM, ADMIN_HOLDERS, LOG_CLAIM};

#[test]
fn every_named_holder_is_spawned_and_every_claim_fits() {
    // Each named whole, from this crate, so tools/nix/inputs.py sees the path.
    let src = |p: &str| std::fs::read_to_string(p).unwrap_or_else(|e| panic!("{p}: {e}"));
    let spawned = [
        src("../../src/userspace/capsule_policy/spawn.rs"),
        src("../../src/userspace/capsule_acpi/spawn.rs"),
        src("../../src/userspace/capsule_install/spawn.rs"),
        src("../../src/services/registry/held_table.rs"),
        src("../../src/kernel_core/process_spawn/capsule_spawn/from_vfs/load/baked_apps.rs"),
    ]
    .concat();
    for n in ADMIN_HOLDERS.iter().filter(|n| **n != b"init") {
        assert!(spawned.contains(&format!("\"{}\"", core::str::from_utf8(n).unwrap())), "{n:?}");
    }
    // The longest claim the screen showed whole before, in characters.
    for claim in [ADMIN_CLAIM, LOG_CLAIM] {
        assert!(claim.len() <= 52, "{}", String::from_utf8_lossy(claim));
    }
}
