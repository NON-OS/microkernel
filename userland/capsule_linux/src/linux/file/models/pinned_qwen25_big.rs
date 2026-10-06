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
 * Qwen2.5 Instruct at Q4_K_M, 14B in three files and 32B in five, from the
 * Qwen/Qwen2.5-*-Instruct-GGUF repositories. Every part is pinned; the
 * program opens the first and finds the rest by their split names.
 */

use super::hex::hex32;
use super::pinned::Pinned;

pub const QWEN25_BIG: &[Pinned] = &[
    Pinned {
        tier: "xxl",
        name: b"/qwen2.5-14b-instruct-q4_k_m-00001-of-00003.gguf",
        bytes: 3_991_999_872,
        sha256: hex32(b"a09ea5e7b1eafb1b30b241726c3cc3c905c96f14ad41e246ffa5f44e53904f68"),
    },
    Pinned {
        tier: "xxl",
        name: b"/qwen2.5-14b-instruct-q4_k_m-00002-of-00003.gguf",
        bytes: 3_989_373_504,
        sha256: hex32(b"21b9457d079680d284e90ef69607c4b2d8ef64a09d4729cb7b5e1357bdba41ae"),
    },
    Pinned {
        tier: "xxl",
        name: b"/qwen2.5-14b-instruct-q4_k_m-00003-of-00003.gguf",
        bytes: 1_006_737_120,
        sha256: hex32(b"c8d37006760a387a35216e070e6664d7da927f10be8eb870fef2e3d4833d9976"),
    },
    Pinned {
        tier: "max",
        name: b"/qwen2.5-32b-instruct-q4_k_m-00001-of-00005.gguf",
        bytes: 3_961_498_272,
        sha256: hex32(b"403434e5c845452c013661d586b97d5a53cf207462d180a07c301eafa9390d05"),
    },
    Pinned {
        tier: "max",
        name: b"/qwen2.5-32b-instruct-q4_k_m-00002-of-00005.gguf",
        bytes: 3_948_996_064,
        sha256: hex32(b"7371e5c5d717a8c20f526dbfce5d3f201dc3f02140d1c836314cd0756e02e8d7"),
    },
    Pinned {
        tier: "max",
        name: b"/qwen2.5-32b-instruct-q4_k_m-00003-of-00005.gguf",
        bytes: 3_993_478_688,
        sha256: hex32(b"f023ccc294c3bd3b2eac5b2dd40dea3aa4ca1d06c7460ead67c36386cd62e8fc"),
    },
    Pinned {
        tier: "max",
        name: b"/qwen2.5-32b-instruct-q4_k_m-00004-of-00005.gguf",
        bytes: 3_950_347_744,
        sha256: hex32(b"05fe76d941454390cd7aa0de3b342f83a1d1226a959479af85bbfae7cd93e771"),
    },
    Pinned {
        tier: "max",
        name: b"/qwen2.5-32b-instruct-q4_k_m-00005-of-00005.gguf",
        bytes: 3_997_015_616,
        sha256: hex32(b"8c2e8ecc686129c37821bd9f3b3e251e6ae80deadd3d3348f7e3e84492a63c24"),
    },
];
