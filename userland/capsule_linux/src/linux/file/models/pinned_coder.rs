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
 * Qwen2.5-Coder Instruct at Q4_K_M, 1.5B to 32B, from the
 * Qwen/Qwen2.5-Coder-*-Instruct-GGUF repositories; the larger ones in
 * parts, each pinned.
 *
 * The parts of 7B, 14B and 32B are kept under shorter names than the Qwen
 * team publishes them under, without "-instruct": theirs are 52 to 54
 * bytes, and the data volume keeps a name of at most 48 (see
 * `pinned::keepable`), so under their own names they could never be
 * fetched or imported. The bytes and digests are the published ones; the
 * name they are downloaded by is kept in tools/nonos_qwen_tier/upstream.py.
 * Each short name still ends in "-0000N-of-0000M.gguf", which is how the
 * chat program finds the other parts from the first.
 */

use super::hex::hex32;
use super::pinned::Pinned;

pub const CODER: &[Pinned] = &[
    Pinned {
        tier: "coder-1.5b",
        name: b"/qwen2.5-coder-1.5b-instruct-q4_k_m.gguf",
        bytes: 1_117_320_768,
        sha256: hex32(b"cc324af070c2ecbfd324a30884d2f951a7ff756aba85cb811a6ec436933bb046"),
    },
    Pinned {
        tier: "coder-7b",
        name: b"/qwen2.5-coder-7b-q4_k_m-00001-of-00002.gguf",
        bytes: 3_993_201_376,
        sha256: hex32(b"89f120544682078148c5a86117de9af3a65c339111262f2d3ff01d80d48b14be"),
    },
    Pinned {
        tier: "coder-7b",
        name: b"/qwen2.5-coder-7b-q4_k_m-00002-of-00002.gguf",
        bytes: 689_872_288,
        sha256: hex32(b"0183b3c850cfa96c31082c3af0123115300d3f62798c4448fa8f57bd0eac05e0"),
    },
    Pinned {
        tier: "coder-14b",
        name: b"/qwen2.5-coder-14b-q4_k_m-00001-of-00002.gguf",
        bytes: 8_000_444_480,
        sha256: hex32(b"310a553a856d7b238c05ed1b0cb877c4dca5b65c2286324854a1397149a00af2"),
    },
    Pinned {
        tier: "coder-14b",
        name: b"/qwen2.5-coder-14b-q4_k_m-00002-of-00002.gguf",
        bytes: 987_665_920,
        sha256: hex32(b"7a52538b39090d99ce93b1b05a406b805427171f890dbef8684c57fb28b2d96a"),
    },
    Pinned {
        tier: "coder-32b",
        name: b"/qwen2.5-coder-32b-q4_k_m-00001-of-00003.gguf",
        bytes: 7_990_120_512,
        sha256: hex32(b"0a9145d25318b7584094e77044cc256bc9f1374c8f168a3812119242b456f69b"),
    },
    Pinned {
        tier: "coder-32b",
        name: b"/qwen2.5-coder-32b-q4_k_m-00002-of-00003.gguf",
        bytes: 7_943_826_304,
        sha256: hex32(b"de1e27aa436e0856582eed095418fd3db8538b0d5d6e71b6362208b3a7f6d16f"),
    },
    Pinned {
        tier: "coder-32b",
        name: b"/qwen2.5-coder-32b-q4_k_m-00003-of-00003.gguf",
        bytes: 3_917_389_312,
        sha256: hex32(b"4d893bec57ae6b2c898c0f2f0f9804a5d855dc7091255b76f3671cb8787919fe"),
    },
];
