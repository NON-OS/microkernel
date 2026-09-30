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
 * Qwen2.5 Instruct at Q4_K_M, 0.5B to 7B, from the
 * Qwen/Qwen2.5-*-Instruct-GGUF repositories. The 7B model comes as two
 * files, both pinned.
 */

use super::hex::hex32;
use super::pinned::Pinned;

pub const QWEN25: &[Pinned] = &[
    Pinned {
        tier: "small",
        name: b"/qwen2.5-0.5b-instruct-q4_k_m.gguf",
        bytes: 491_400_032,
        sha256: hex32(b"74a4da8c9fdbcd15bd1f6d01d621410d31c6fc00986f5eb687824e7b93d7a9db"),
    },
    Pinned {
        tier: "medium",
        name: b"/qwen2.5-1.5b-instruct-q4_k_m.gguf",
        bytes: 1_117_320_736,
        sha256: hex32(b"6a1a2eb6d15622bf3c96857206351ba97e1af16c30d7a74ee38970e434e9407e"),
    },
    Pinned {
        tier: "large",
        name: b"/qwen2.5-3b-instruct-q4_k_m.gguf",
        bytes: 2_104_932_768,
        sha256: hex32(b"626b4a6678b86442240e33df819e00132d3ba7dddfe1cdc4fbb18e0a9615c62d"),
    },
    Pinned {
        tier: "xlarge",
        name: b"/qwen2.5-7b-instruct-q4_k_m-00001-of-00002.gguf",
        bytes: 3_993_201_344,
        sha256: hex32(b"dfce12e3862a5283ccfb88221b48480e58745165de856439950d0f22590580db"),
    },
    Pinned {
        tier: "xlarge",
        name: b"/qwen2.5-7b-instruct-q4_k_m-00002-of-00002.gguf",
        bytes: 689_872_288,
        sha256: hex32(b"539cf93f78e887edea1c04e2d7d8cdaca9d01dae9c9025bcb8accbe29df3d72a"),
    },
];
