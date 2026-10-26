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
 * Qwen3-VL 2B Instruct, from the Qwen team's own
 * Qwen/Qwen3-VL-2B-Instruct-GGUF repository (Apache-2.0): the model at
 * Q4_K_M and its vision projector (mmproj) at Q8_0, each digest the one its
 * LFS pointer names. Both are needed: qwenchat opens the projector beside
 * the model and shows it the image (qwenchat_vision.cpp).
 */

use super::hex::hex32;
use super::pinned::Pinned;

pub const VL: &[Pinned] = &[
    Pinned {
        tier: "vl-2b",
        name: b"/Qwen3VL-2B-Instruct-Q4_K_M.gguf",
        bytes: 1_107_409_952,
        sha256: hex32(b"089d75c52f4b7ffc56ba998ffc50aae89fcafc755f9e7208aacca281dca6c2ae"),
    },
    Pinned {
        tier: "vl-2b",
        name: b"/mmproj-Qwen3VL-2B-Instruct-Q8_0.gguf",
        bytes: 445_053_216,
        sha256: hex32(b"f9a68fabba69c3b81e153367b2c7521030b0fa8bb0de400c9599c8e6725f9c82"),
    },
];
