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

/*
 * The shape of every tier, for need.rs's rule. Each shape is (layers, KV
 * heads, head width, embedding width) from the model's config.json in the
 * Qwen team's Hugging Face repository (Qwen/<model>),
 * read on 2026-09-30, or, for a tier pinned from ggml-org, from the same
 * keys in its GGUF file's header (block_count, attention.head_count_kv,
 * attention.key_length, embedding_length), which is what qwenchat reads,
 * read on 2026-10-25. Qwen3.5 and Qwen3.6 attend in one layer in four; the
 * rule counts every layer, as qwenchat does, so their KV need is about four
 * times their cache.
 *
 * tools/nonos_qwen_tier/shapes.py reads this file: the constants and the
 * rows, one a line.
 */

pub const POSITIONS: u64 = 2048;
pub const BATCH: u64 = 512;
pub const MARGIN: u64 = 64 << 20;

#[rustfmt::skip]
pub const SHAPES: &[(&str, u64, u64, u64, u64)] = &[
    ("small", 24, 2, 64, 896),             // Qwen2.5-0.5B-Instruct
    ("medium", 28, 2, 128, 1536),          // Qwen2.5-1.5B-Instruct
    ("large", 36, 2, 128, 2048),           // Qwen2.5-3B-Instruct
    ("xlarge", 28, 4, 128, 3584),          // Qwen2.5-7B-Instruct
    ("xxl", 48, 8, 128, 5120),             // Qwen2.5-14B-Instruct
    ("max", 64, 8, 128, 5120),             // Qwen2.5-32B-Instruct
    ("qwen3-0.6b", 28, 8, 128, 1024),      // Qwen3-0.6B
    ("qwen3-1.7b", 28, 8, 128, 2048),      // Qwen3-1.7B
    ("qwen3-4b", 36, 8, 128, 2560),        // Qwen3-4B
    ("qwen3-8b", 36, 8, 128, 4096),        // Qwen3-8B
    ("qwen3-14b", 40, 8, 128, 5120),       // Qwen3-14B
    ("qwen3-30b-a3b", 48, 4, 128, 2048),   // Qwen3-30B-A3B
    ("qwen3-32b", 64, 8, 128, 5120),       // Qwen3-32B
    ("qwen3.5-0.8b", 25, 2, 256, 1024),    // Qwen3.5-0.8B, from its GGUF
    ("qwen3.6-27b", 64, 4, 256, 5120),     // Qwen3.6-27B, from its GGUF
    ("coder-1.5b", 28, 2, 128, 1536),      // Qwen2.5-Coder-1.5B-Instruct
    ("coder-7b", 28, 4, 128, 3584),        // Qwen2.5-Coder-7B-Instruct
    ("coder-14b", 48, 8, 128, 5120),       // Qwen2.5-Coder-14B-Instruct
    ("coder-32b", 64, 8, 128, 5120),       // Qwen2.5-Coder-32B-Instruct
];
