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

//! The shipped tiers of Qwen2.5-Coder Instruct, 1.5B to 32B.

use super::apps::{tier, App};

pub const CODER: &[App] = &[
    tier!("qwen-coder-1.5b", "/qwen2.5-coder-1.5b-instruct-q4_k_m.gguf"),
    tier!(
        "qwen-coder-7b",
        "/qwen2.5-coder-7b-q4_k_m-00001-of-00002.gguf",
        "/qwen2.5-coder-7b-q4_k_m-00002-of-00002.gguf"
    ),
    tier!(
        "qwen-coder-14b",
        "/qwen2.5-coder-14b-q4_k_m-00001-of-00002.gguf",
        "/qwen2.5-coder-14b-q4_k_m-00002-of-00002.gguf"
    ),
    tier!(
        "qwen-coder-32b",
        "/qwen2.5-coder-32b-q4_k_m-00001-of-00003.gguf",
        "/qwen2.5-coder-32b-q4_k_m-00002-of-00003.gguf",
        "/qwen2.5-coder-32b-q4_k_m-00003-of-00003.gguf"
    ),
];
