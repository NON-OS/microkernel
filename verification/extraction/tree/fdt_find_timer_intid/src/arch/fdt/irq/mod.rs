// NONOS Operating System (AGPL-3.0-or-later)
//
// Mirrors src/arch/fdt/irq/mod.rs for the one child the extraction needs. The
// kernel's `parent` and `specifier` are methods on `Fdt` that walk the parsed
// tree, so they are left out rather than stubbed; `gic` reads only the
// specifier bytes it is handed.

#[path = "../../../../../../../../src/arch/fdt/irq/gic.rs"]
mod gic;

pub use gic::{gic_edge, gic_intid};
