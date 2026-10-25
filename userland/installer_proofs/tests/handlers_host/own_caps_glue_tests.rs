// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The installer reads a manifest's head with its own copy of the kernel's
//! layout constants; they must stay the kernel's, or the set it asks for
//! drifts from the one the kernel checks.

use crate::{CAPS_OF_APP_INSTALL as INSTALL, CAPS_OF_SPAWN_WINDOW as WINDOW};

const KERNEL: &str = include_str!("../../../../src/security/capsule_manifest/schema/constants.rs");
const OWN: &str =
    include_str!("../../../capsule_installer/src/server/handlers/load_by_name/own_caps.rs");

fn value(src: &str, name: &str) -> String {
    let line = src.lines().find(|l| l.contains(&format!("const {name}:"))).expect(name);
    line.split('=').nth(1).expect("a value").trim().trim_end_matches(';').to_string()
}

#[test]
fn the_installer_reads_the_head_with_the_kernel_s_layout() {
    for (own, kernel) in [
        ("SCHEMA", "MANIFEST_SCHEMA_VERSION"),
        ("CERT_ID", "NONOS_ID_CERT_ID_LEN"),
        ("PAYLOAD_HASH", "PAYLOAD_HASH_LEN"),
    ] {
        assert_eq!(value(OWN, own), value(KERNEL, kernel), "{own} against the kernel's {kernel}");
    }
}

#[test]
fn libc_names_the_kernel_s_two_askable_bits() {
    let libc = include_str!("../../../libc/src/process.rs");
    let kernel = include_str!("../../../../src/syscall/microkernel/caps_of/rule.rs");
    assert!(libc.contains("pub const CAPS_OF_SPAWN_WINDOW: u64 = 1 << 24;"));
    assert!(libc.contains("pub const CAPS_OF_APP_INSTALL: u64 = 1 << 30;"));
    assert!(kernel.contains("[Capability::SpawnWindow.bit(), Capability::AppInstall.bit()]"));
    assert_eq!((WINDOW, INSTALL), (1 << 24, 1 << 30), "the stand-in's are libc's");
}
