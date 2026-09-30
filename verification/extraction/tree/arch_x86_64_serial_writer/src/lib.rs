// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/arch/x86_64/serial/writer.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;

pub fn serialwriter_new() -> crate::arch::x86_64::serial::writer::SerialWriter {
    crate::arch::x86_64::serial::writer::SerialWriter::new()
}

pub fn serialwriter_for_port(port_index: usize) -> crate::arch::x86_64::serial::writer::SerialWriter {
    crate::arch::x86_64::serial::writer::SerialWriter::for_port(port_index)
}

pub fn serialwriter_port_index(this: crate::arch::x86_64::serial::writer::SerialWriter) -> usize {
    this.port_index()
}

