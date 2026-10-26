// NONOS Operating System (AGPL-3.0-or-later)
#[path = "../../../../../src/arch/fdt/endian.rs"]
pub mod endian;
#[path = "../../../../../src/arch/fdt/error.rs"]
pub mod error;
pub mod find;
#[path = "../../../../../src/arch/fdt/header.rs"]
pub mod header;
#[path = "../../../../../src/arch/fdt/layout.rs"]
pub mod layout;
#[path = "../../../../../src/arch/fdt/limits.rs"]
pub mod limits;
#[path = "../../../../../src/arch/fdt/parser.rs"]
pub mod parser;
#[path = "../../../../../src/arch/fdt/property/mod.rs"]
pub mod property;
#[path = "../../../../../src/arch/fdt/strings.rs"]
pub mod strings;
#[path = "../../../../../src/arch/fdt/tokens.rs"]
pub mod tokens;
#[path = "../../../../../src/arch/fdt/validate.rs"]
mod validate;
#[path = "../../../../../src/arch/fdt/walker/mod.rs"]
pub mod walker;

pub(crate) mod blob;
mod i2c_describe_tests;
