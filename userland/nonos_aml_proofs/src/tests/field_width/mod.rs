// NONOS Operating System (AGPL-3.0-or-later)
//! Field units at any bit offset and length, under every AccessType and
//! UpdateRule, in memory, I/O and an embedded controller, against a byte
//! model: the accesses made, the value read, and the bytes after a write.

mod bank_tests;
mod gen;
mod gen_ib;
mod index_tests;
mod io;
mod model;
mod model_ib;
mod sweep_tests;
mod width;
