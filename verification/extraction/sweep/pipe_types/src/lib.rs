// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/ipc/pipe/types.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/ipc/pipe/types.rs"]
pub mod types;

pub fn pipe_new(id: u32, capacity: usize) -> types::Pipe {
    types::Pipe::new(id, capacity)
}

pub fn pipe_pipe_id(this: types::Pipe) -> u32 {
    this.pipe_id()
}

pub fn pipe_is_broken(this: types::Pipe) -> bool {
    this.is_broken()
}

pub fn pipe_space_available(this: types::Pipe) -> usize {
    this.space_available()
}


// types.rs zeroes a pipe's buffer on drop through crate::crypto::secure_zero.
// The kernel's version pulls in the hash module and writes through
// write_volatile, which Aeneas cannot translate, so the mirror zeroes with a
// plain loop. Only the drop path reaches it; no extracted entry point does.
pub mod crypto {
    pub fn secure_zero(data: &mut [u8]) {
        let mut i = 0;
        while i < data.len() {
            data[i] = 0;
            i += 1;
        }
    }
}
