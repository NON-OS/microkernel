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

//! nonos_linux_policy's vendor table is a default feature, off in the crates
//! that read only its codec (nonos.prove, the receipt), so they build without
//! the Linux userland it is made from; capsule_linux, its reader, keeps it.
/// The table, and so the guard, is off only for a crate that reads none:
/// capsule_linux, its one reader, takes the default and so the table.
#[test]
fn capsule_linux_keeps_the_vendor_table() {
    let linux = include_str!("../../../../capsule_linux/Cargo.toml");
    let dep = linux.lines().find(|l| l.starts_with("nonos_linux_policy = ")).unwrap();
    assert_eq!(dep, r#"nonos_linux_policy = { path = "../nonos_linux_policy" }"#);
    let build = include_str!("../../../../nonos_linux_policy/build.rs");
    assert!(build.contains("if env::var_os(\"CARGO_FEATURE_VENDOR\").is_none() {\n        return;"));
    assert!(include_str!("../../../../nonos_linux_policy/Cargo.toml").contains("default = [\"vendor\"]"));
}
