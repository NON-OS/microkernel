// NONOS Operating System (AGPL-3.0-or-later)
#[path = "../../../../../src/arch/fdt/endian.rs"]
pub mod endian;
#[path = "../../../../../src/arch/fdt/error.rs"]
pub mod error;
pub mod find;
#[path = "../../../../../src/arch/fdt/header.rs"]
pub mod header;
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[path = "../../../../../src/arch/fdt/parser.rs"]
pub mod parser;
#[allow(clippy::module_inception)]
#[path = "../../../../../src/arch/fdt/property/mod.rs"]
pub mod property;
#[path = "../../../../../src/arch/fdt/strings.rs"]
pub mod strings;
#[path = "../../../../../src/arch/fdt/tokens.rs"]
pub mod tokens;
#[allow(clippy::module_inception, clippy::should_implement_trait)]
#[allow(clippy::manual_is_multiple_of)]
#[path = "../../../../../src/arch/fdt/walker/mod.rs"]
pub mod walker;

mod blob;
mod i2c_more_tests;
mod i2c_tests;
