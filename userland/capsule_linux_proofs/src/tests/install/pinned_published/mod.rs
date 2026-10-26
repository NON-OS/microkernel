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

//! The model files their publishers give, written out here from the LFS
//! pointers of the Hugging Face GGUF repositories (the Qwen team's, and
//! ggml-org's for Qwen3.5 and Qwen3.6), independently of the personality's
//! tables, so a digest changed there fails a proof here. In table order:
//! Qwen2.5, its 14B and 32B, Qwen3, Qwen3.5 and Qwen3.6, then Coder.

mod kept;
mod published;

pub use kept::{kept_as, KEPT_AS, TIERS};
pub use published::PUBLISHED;
