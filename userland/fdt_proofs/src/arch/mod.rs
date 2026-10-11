// NONOS Operating System (AGPL-3.0-or-later)
// The parser keeps `walker/walker.rs` and `property/property.rs`, and its
// `Walker::next` returns a `Result` an `Iterator` could not.
#[allow(clippy::module_inception, clippy::should_implement_trait)]
#[path = "../../../../src/arch/fdt/mod.rs"]
pub mod fdt;
