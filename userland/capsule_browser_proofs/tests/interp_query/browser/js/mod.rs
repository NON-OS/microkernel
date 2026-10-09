// NONOS Operating System (AGPL-3.0-or-later)
//! The capsule's script interpreter, minus the script collector, which reads
//! the HTML parser this test has no use for.

#[path = "../../../../../capsule_browser/src/browser/js/ast/mod.rs"]
pub mod ast;
#[path = "../../../../../capsule_browser/src/browser/js/env.rs"]
pub mod env;
#[path = "../../../../../capsule_browser/src/browser/js/interp/mod.rs"]
pub mod interp;
#[path = "../../../../../capsule_browser/src/browser/js/regex/mod.rs"]
pub mod regex;
#[path = "../../../../../capsule_browser/src/browser/js/value.rs"]
pub mod value;
#[path = "../../../../../capsule_browser/src/browser/js/world.rs"]
pub mod world;
