// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! A strict reader of GGUF model headers. A model file is hostile input until
//! it is checked: its counts, lengths and tensor shapes size everything an
//! engine allocates, so each is bounded here first and a file that breaks a
//! bound is refused with the field and the value that broke it.

#![no_std]

extern crate alloc;
#[cfg(test)]
extern crate std;

mod cursor;
mod cursor_read;
mod error;
mod layout;
mod limits;
mod meta;
mod parse;
mod source;
mod summary;
mod tensor;
mod tensor_error;
mod tensor_size;
mod text;
mod types;
mod value;
mod value_array;

#[cfg(test)]
mod tests;

pub use error::GgufError;
pub use limits::{Limits, DEFAULT};
pub use meta::DEFAULT_ALIGNMENT;
pub use parse::parse;
pub use source::ReadAt;
pub use summary::{Meta, Summary};
pub use tensor::MAX_DIMS;
pub use tensor_error::TensorError;
pub use text::{Text, TEXT_BYTES};
pub use types::TYPE_SLOTS;
