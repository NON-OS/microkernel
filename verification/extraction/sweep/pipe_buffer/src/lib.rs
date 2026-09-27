// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/fs/pipe/buffer.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/fs/pipe/buffer.rs"]
pub mod buffer;

pub fn pipebuffer_new() -> buffer::PipeBuffer {
    buffer::PipeBuffer::new()
}

pub fn pipebuffer_len(this: buffer::PipeBuffer) -> usize {
    this.len()
}

pub fn pipebuffer_is_empty(this: buffer::PipeBuffer) -> bool {
    this.is_empty()
}

pub fn pipebuffer_available_write(this: buffer::PipeBuffer) -> usize {
    this.available_write()
}

pub fn pipebuffer_add_reader(this: buffer::PipeBuffer) {
    this.add_reader()
}

pub fn pipebuffer_remove_reader(this: buffer::PipeBuffer) {
    this.remove_reader()
}

pub fn pipebuffer_add_writer(this: buffer::PipeBuffer) {
    this.add_writer()
}

pub fn pipebuffer_remove_writer(this: buffer::PipeBuffer) {
    this.remove_writer()
}

pub fn pipebuffer_has_readers(this: buffer::PipeBuffer) -> bool {
    this.has_readers()
}

pub fn pipebuffer_has_writers(this: buffer::PipeBuffer) -> bool {
    this.has_writers()
}

