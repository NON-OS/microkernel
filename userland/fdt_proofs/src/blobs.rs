// NONOS Operating System (AGPL-3.0-or-later)
//! Shared helpers for tests that patch a blob's header.

use crate::arch::fdt::{Fdt, FdtError};
use crate::builder::{Builder, END};

pub fn minimal() -> Vec<u8> {
    Builder::default().begin("").end().token(END).finish()
}

pub fn set(blob: &mut [u8], field: usize, value: u32) {
    blob[field * 4..field * 4 + 4].copy_from_slice(&value.to_be_bytes());
}

pub fn err(blob: &[u8]) -> FdtError {
    Fdt::from_bytes(blob).err().expect("blob should be refused")
}
