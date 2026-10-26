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
 * Qwen3.5 0.8B at Q8_0 and Qwen3.6 27B at Q4_K_M. The Qwen team publishes
 * Qwen3.5 and Qwen3.6 in safetensors only; these GGUF files are the ones
 * ggml-org, the llama.cpp project, converts from them and publishes in its
 * ggml-org/Qwen3.5-0.8B-GGUF and ggml-org/Qwen3.6-27B-GGUF repositories,
 * each digest the one its LFS pointer names, both Apache-2.0. Both are
 * hybrid models: one layer in four attends, the rest are linear (Gated
 * DeltaNet), so the KV cache need.rs counts for every layer is four times
 * what they hold, and the room it leaves covers their recurrent state.
 */

use super::hex::hex32;
use super::pinned::Pinned;

pub const QWEN35: &[Pinned] = &[
    Pinned {
        tier: "qwen3.5-0.8b",
        name: b"/Qwen3.5-0.8B-Q8_0.gguf",
        bytes: 833_592_096,
        sha256: hex32(b"37ae482d336108d23516fa35e8e0c4126688d81018b87178a18d752a1357814f"),
    },
    Pinned {
        tier: "qwen3.6-27b",
        name: b"/Qwen3.6-27B-Q4_K_M.gguf",
        bytes: 19_095_766_304,
        sha256: hex32(b"65b753ea835627f7b511143c6ceb976525c7f21f5df8c664bc0a9c23d1c49921"),
    },
];
