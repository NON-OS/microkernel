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

//! The shipped tiers of Qwen3, 0.6B to 32B, and 30B-A3B, a mixture of experts.

use super::apps::{tier, App};

pub const QWEN3: &[App] = &[
    tier!("qwen-qwen3-0.6b", "/Qwen3-0.6B-Q8_0.gguf"),
    tier!("qwen-qwen3-1.7b", "/Qwen3-1.7B-Q8_0.gguf"),
    tier!("qwen-qwen3-4b", "/Qwen3-4B-Q4_K_M.gguf"),
    tier!("qwen-qwen3-8b", "/Qwen3-8B-Q4_K_M.gguf"),
    tier!("qwen-qwen3-14b", "/Qwen3-14B-Q4_K_M.gguf"),
    tier!("qwen-qwen3-30b-a3b", "/Qwen3-30B-A3B-Q4_K_M.gguf"),
    tier!("qwen-qwen3-32b", "/Qwen3-32B-Q4_K_M.gguf"),
];
