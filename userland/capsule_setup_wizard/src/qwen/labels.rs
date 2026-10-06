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
 * What each pinned tier is called on screen: its family and parameter
 * count, which the pins do not hold. pins.rs stops the build when a tier is
 * pinned with no name here, or named here and no longer pinned.
 */

pub const LABELS: &[(&[u8], &[u8])] = &[
    (b"small", b"Qwen2.5 0.5B"),
    (b"medium", b"Qwen2.5 1.5B"),
    (b"large", b"Qwen2.5 3B"),
    (b"xlarge", b"Qwen2.5 7B"),
    (b"xxl", b"Qwen2.5 14B"),
    (b"max", b"Qwen2.5 32B"),
    (b"qwen3-0.6b", b"Qwen3 0.6B"),
    (b"qwen3-1.7b", b"Qwen3 1.7B"),
    (b"qwen3-4b", b"Qwen3 4B"),
    (b"qwen3-8b", b"Qwen3 8B"),
    (b"qwen3-14b", b"Qwen3 14B"),
    (b"qwen3-30b-a3b", b"Qwen3 30B-A3B"),
    (b"qwen3-32b", b"Qwen3 32B"),
    (b"coder-1.5b", b"Qwen2.5 Coder 1.5B"),
    (b"coder-7b", b"Qwen2.5 Coder 7B"),
    (b"coder-14b", b"Qwen2.5 Coder 14B"),
    (b"coder-32b", b"Qwen2.5 Coder 32B"),
];

pub fn label(tier: &[u8]) -> &[u8] {
    LABELS.iter().find(|(t, _)| *t == tier).map_or(tier, |(_, name)| name)
}
