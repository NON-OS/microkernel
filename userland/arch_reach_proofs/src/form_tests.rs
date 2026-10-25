// NONOS Operating System (AGPL-3.0-or-later)
//! The ways round the plain forms: a gate written as `not` of the other
//! architectures, and `x86_64::` reached through a `crate::arch::{}` group,
//! on one line or spread over several. Each counts as the plain form does.

use crate::scan::{hits, names_x86_cfg, names_x86_path, Rule};

#[test]
fn the_roundabout_forms_count_too() {
    let neg = "#[cfg(not(any(target_arch = \"aarch64\", target_arch = \"riscv64\")))]";
    assert!(names_x86_cfg(neg));
    assert!(!names_x86_cfg("#[cfg(not(feature = \"aarch64\"))]"));
    assert!(names_x86_path("use crate::arch::{x86_64::vga, Arch};"));
    let group = "use crate::arch::{\n    Arch,\n    x86_64::cpu::halt,\n};\nx86_64::x();\n";
    assert_eq!(hits(group, Rule::Path), 1);
    assert!(!names_x86_path("use crate::arch::{Arch, ArchOps};"));
}
