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

//! The shipped tiers of Qwen2.5 Instruct, 0.5B to 32B.

use super::apps::{tier, App};

pub const QWEN25: &[App] = &[
    tier!("qwen-small", "/qwen2.5-0.5b-instruct-q4_k_m.gguf"),
    tier!("qwen-medium", "/qwen2.5-1.5b-instruct-q4_k_m.gguf"),
    tier!("qwen-large", "/qwen2.5-3b-instruct-q4_k_m.gguf"),
    tier!(
        "qwen-xlarge",
        "/qwen2.5-7b-instruct-q4_k_m-00001-of-00002.gguf",
        "/qwen2.5-7b-instruct-q4_k_m-00002-of-00002.gguf"
    ),
    tier!(
        "qwen-xxl",
        "/qwen2.5-14b-instruct-q4_k_m-00001-of-00003.gguf",
        "/qwen2.5-14b-instruct-q4_k_m-00002-of-00003.gguf",
        "/qwen2.5-14b-instruct-q4_k_m-00003-of-00003.gguf"
    ),
    tier!(
        "qwen-max",
        "/qwen2.5-32b-instruct-q4_k_m-00001-of-00005.gguf",
        "/qwen2.5-32b-instruct-q4_k_m-00002-of-00005.gguf",
        "/qwen2.5-32b-instruct-q4_k_m-00003-of-00005.gguf",
        "/qwen2.5-32b-instruct-q4_k_m-00004-of-00005.gguf",
        "/qwen2.5-32b-instruct-q4_k_m-00005-of-00005.gguf"
    ),
];
