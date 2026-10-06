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
 * Qwen3, 0.6B to 32B, from the Qwen/Qwen3-*-GGUF repositories: the two
 * smallest at Q8_0, the only quantisation published for them, the rest at
 * Q4_K_M. 30B-A3B is a mixture of experts with 3B active per token, so it
 * answers far faster on a CPU than its size suggests.
 */

use super::hex::hex32;
use super::pinned::Pinned;

pub const QWEN3: &[Pinned] = &[
    Pinned {
        tier: "qwen3-0.6b",
        name: b"/Qwen3-0.6B-Q8_0.gguf",
        bytes: 639_446_688,
        sha256: hex32(b"9465e63a22add5354d9bb4b99e90117043c7124007664907259bd16d043bb031"),
    },
    Pinned {
        tier: "qwen3-1.7b",
        name: b"/Qwen3-1.7B-Q8_0.gguf",
        bytes: 1_834_426_016,
        sha256: hex32(b"061b54daade076b5d3362dac252678d17da8c68f07560be70818cace6590cb1a"),
    },
    Pinned {
        tier: "qwen3-4b",
        name: b"/Qwen3-4B-Q4_K_M.gguf",
        bytes: 2_497_280_256,
        sha256: hex32(b"7485fe6f11af29433bc51cab58009521f205840f5b4ae3a32fa7f92e8534fdf5"),
    },
    Pinned {
        tier: "qwen3-8b",
        name: b"/Qwen3-8B-Q4_K_M.gguf",
        bytes: 5_027_783_488,
        sha256: hex32(b"d98cdcbd03e17ce47681435b5150e34c1417f50b5c0019dd560e4882c5745785"),
    },
    Pinned {
        tier: "qwen3-14b",
        name: b"/Qwen3-14B-Q4_K_M.gguf",
        bytes: 9_001_752_960,
        sha256: hex32(b"500a8806e85ee9c83f3ae08420295592451379b4f8cf2d0f41c15dffeb6b81f0"),
    },
    Pinned {
        tier: "qwen3-30b-a3b",
        name: b"/Qwen3-30B-A3B-Q4_K_M.gguf",
        bytes: 18_556_685_824,
        sha256: hex32(b"0d003f6662faee786ed5da3e31b29c978de5ae5d275c8794c606a7f3c01aa8f5"),
    },
    Pinned {
        tier: "qwen3-32b",
        name: b"/Qwen3-32B-Q4_K_M.gguf",
        bytes: 19_762_149_024,
        sha256: hex32(b"efd971561896866f0e910cce52761ca77b1b138090c7f15fe284676d57d1f689"),
    },
];
