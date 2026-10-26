// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for the aarch64 UEFI loader. The modules below are the
//! loader's own files; `walk` assembles the loader's own `walk.S`.

#![allow(dead_code)]

#[path = "../../../nonos-bootloader/src/loader/core/exec/block_layout.rs"]
pub mod block_layout;
#[path = "../../../nonos-bootloader/src/arch/aarch64/fdt_header.rs"]
pub mod fdt_header;
#[path = "../../../nonos-bootloader/src/arch/aarch64/caches/geometry.rs"]
pub mod geometry;
#[path = "../../../nonos-bootloader/src/arch/aarch64/exit/handed.rs"]
pub mod handed;
#[path = "../../../nonos-bootloader/src/arch/aarch64/cpu/id_fields.rs"]
pub mod id_fields;
#[path = "../../../nonos-bootloader/src/arch/aarch64/cpu/image_header.rs"]
pub mod image_header;
#[path = "../../../nonos-bootloader/src/loader/core/exec/image_rules.rs"]
pub mod image_rules;
#[path = "../../../nonos-bootloader/src/entropy/strong_rule.rs"]
pub mod strong_rule;
#[cfg(target_arch = "aarch64")]
pub mod walk;

#[cfg(test)]
mod enter_source_tests;
#[cfg(test)]
mod header_tests;
#[cfg(test)]
mod image_rules_tests;
#[cfg(test)]
mod layout_tests;
#[cfg(test)]
mod rule_tests;
#[cfg(test)]
mod source;
#[cfg(all(test, target_arch = "aarch64"))]
mod walk_model;
#[cfg(test)]
mod walk_source_tests;
#[cfg(all(test, target_arch = "aarch64"))]
mod walk_tests;
#[cfg(all(test, target_arch = "aarch64"))]
mod walk_width_tests;
