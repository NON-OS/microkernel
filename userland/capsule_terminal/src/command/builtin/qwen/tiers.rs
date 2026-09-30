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

//! The tier words `qwen` takes, as the kernel's allowlist has them.

/// The tiers the kernel runs, by family and smallest first within each.
/// With none named, the first: Qwen2.5 0.5B.
pub const TIERS: &[&[u8]] = &[
    b"small",
    b"medium",
    b"large",
    b"xlarge",
    b"xxl",
    b"max",
    b"qwen3-0.6b",
    b"qwen3-1.7b",
    b"qwen3-4b",
    b"qwen3-8b",
    b"qwen3-14b",
    b"qwen3-30b-a3b",
    b"qwen3-32b",
    b"coder-1.5b",
    b"coder-7b",
    b"coder-14b",
    b"coder-32b",
];

/* The tier policy names in `want`, or the first when it names none this terminal knows. */
pub fn pick(want: &[u8]) -> &'static [u8] {
    TIERS.iter().copied().find(|t| *t == want).unwrap_or(TIERS[0])
}
