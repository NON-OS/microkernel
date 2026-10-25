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

//! The panic path's place in the kernel, read from the source: the soft-off
//! read never waits on the ACPI tables' lock, the halt line comes before the
//! wipe, and the panic proof cannot be built into a release.

const PREPARED: &str = include_str!("../../../../src/arch/power/prepared.rs");
const TABLES: &str = include_str!("../../../../src/arch/x86_64/acpi/parser/getters_table.rs");
const HANDLER: &str = include_str!("../../../../src/boot/panic/handler.rs");
const INJECTED: &str = include_str!("../../../../src/userspace/init/injected.rs");

fn at(text: &str, needle: &str) -> usize {
    text.find(needle).unwrap_or_else(|| panic!("missing: {needle}"))
}

#[test]
fn the_soft_off_read_tries_the_tables_lock_and_never_takes_it() {
    assert!(PREPARED.contains("off: prepare_shutdown(try_with_data).ok(),"));
    assert!(!PREPARED.contains("prepare_shutdown(with_data)"));
    let tried = at(TABLES, "pub fn try_with_data<F, R>(f: F) -> Option<R>");
    assert!(TABLES[tried..].contains("TABLES.try_read()?.as_ref().map(|t| f(&t.data))"));
}

#[test]
fn the_handler_says_the_cpus_halted_then_wipes() {
    let halted = at(HANDLER, "if crate::smp::others_halted() {");
    let said = at(HANDLER, "[ZEROSTATE] panic: cpus halted, wiping, powering off");
    let wiped = at(HANDLER, "crate::security::zerostate::panic_wipe_and_off()");
    assert!(halted < said && said < wiped);
}

#[test]
fn the_panic_proof_refuses_a_production_or_release_build() {
    let guard = "#[cfg(all(\n    feature = \"nonos-panic-proof\",\n    any(feature = \"nonos-production\", feature = \"nonos-release\")\n))]\ncompile_error!(";
    assert!(INJECTED.contains(guard), "the guard is gone or weakened");
}
