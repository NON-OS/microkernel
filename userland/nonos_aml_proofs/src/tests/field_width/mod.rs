// NONOS Operating System (AGPL-3.0-or-later)
//! Field units at any bit offset and length, under every AccessType and
//! UpdateRule, in memory, I/O and an embedded controller, against a byte
//! model: the accesses made, the value read, and the bytes after a write.

mod gen;
mod io;
mod model;
mod sweep_tests;
mod width;
