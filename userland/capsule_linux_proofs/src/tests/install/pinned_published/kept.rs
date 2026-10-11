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

//! Where each file is kept, and the tiers that pin them.

/// The published files the data volume keeps under a shorter name, as
/// (published, kept): their own names are longer than the volume keeps.
/// The parts keep their "-0000N-of-0000M.gguf" ending.
#[rustfmt::skip]
pub const KEPT_AS: [(&str, &str); 7] = [
    ("qwen2.5-coder-7b-instruct-q4_k_m-00001-of-00002.gguf", "qwen2.5-coder-7b-q4_k_m-00001-of-00002.gguf"),
    ("qwen2.5-coder-7b-instruct-q4_k_m-00002-of-00002.gguf", "qwen2.5-coder-7b-q4_k_m-00002-of-00002.gguf"),
    ("qwen2.5-coder-14b-instruct-q4_k_m-00001-of-00002.gguf", "qwen2.5-coder-14b-q4_k_m-00001-of-00002.gguf"),
    ("qwen2.5-coder-14b-instruct-q4_k_m-00002-of-00002.gguf", "qwen2.5-coder-14b-q4_k_m-00002-of-00002.gguf"),
    ("qwen2.5-coder-32b-instruct-q4_k_m-00001-of-00003.gguf", "qwen2.5-coder-32b-q4_k_m-00001-of-00003.gguf"),
    ("qwen2.5-coder-32b-instruct-q4_k_m-00002-of-00003.gguf", "qwen2.5-coder-32b-q4_k_m-00002-of-00003.gguf"),
    ("qwen2.5-coder-32b-instruct-q4_k_m-00003-of-00003.gguf", "qwen2.5-coder-32b-q4_k_m-00003-of-00003.gguf"),
];

/// The name the volume keeps a published file under.
pub fn kept_as(published: &str) -> &str {
    KEPT_AS.iter().find(|(p, _)| *p == published).map_or(published, |(_, k)| k)
}

/// Every tier the tables ship, in the same order.
pub const TIERS: [&str; 17] = [
    "small",
    "medium",
    "large",
    "xlarge",
    "xxl",
    "max",
    "qwen3-0.6b",
    "qwen3-1.7b",
    "qwen3-4b",
    "qwen3-8b",
    "qwen3-14b",
    "qwen3-30b-a3b",
    "qwen3-32b",
    "coder-1.5b",
    "coder-7b",
    "coder-14b",
    "coder-32b",
];
