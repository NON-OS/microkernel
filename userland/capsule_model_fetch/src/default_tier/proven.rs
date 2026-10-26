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

use super::super::need::STICK_TIER;

/*
 * The tiers proven on NONOS, each with the boot that proved it: its
 * architecture (as llama.cpp names it) answered a question on that boot.
 * A tier's mark is added in its own commit, naming its evidence, when a
 * tier of its architecture boots to an answer. Not here: qwen3-30b-a3b
 * (qwen3moe), whose smallest published size has not run on NONOS, and
 * vl-2b (qwen3vl), chosen by name only until it answers an image.
 */
#[rustfmt::skip]
pub const PROVEN: &[(&str, &str)] = &[
    ("small", "docs/using/evidence/qwen-arch/README.md: small answered"),
    ("medium", "docs/using/evidence/qwen-arch/README.md: small answered"),
    ("large", "docs/using/evidence/qwen-arch/README.md: small answered"),
    ("xlarge", "docs/using/evidence/qwen-arch/README.md: small answered"),
    ("xxl", "docs/using/evidence/qwen-arch/README.md: small answered"),
    ("max", "docs/using/evidence/qwen-arch/README.md: small answered"),
    ("coder-1.5b", "docs/using/evidence/qwen-arch/README.md: small answered"),
    ("coder-7b", "docs/using/evidence/qwen-arch/README.md: small answered"),
    ("coder-14b", "docs/using/evidence/qwen-arch/README.md: small answered"),
    ("coder-32b", "docs/using/evidence/qwen-arch/README.md: small answered"),
    ("qwen3-0.6b", "docs/using/evidence/qwen-arch-qwen3/README.md: qwen3-0.6b answered"),
    ("qwen3-1.7b", "docs/using/evidence/qwen-arch-qwen3/README.md: qwen3-0.6b answered"),
    ("qwen3-4b", "docs/using/evidence/qwen-arch-qwen3/README.md: qwen3-0.6b answered"),
    ("qwen3-8b", "docs/using/evidence/qwen-arch-qwen3/README.md: qwen3-0.6b answered"),
    ("qwen3-14b", "docs/using/evidence/qwen-arch-qwen3/README.md: qwen3-0.6b answered"),
    ("qwen3-32b", "docs/using/evidence/qwen-arch-qwen3/README.md: qwen3-0.6b answered"),
    ("qwen3.5-0.8b", "docs/using/evidence/qwen-arch-qwen35-fixed/README.md: qwen3.5-0.8b answered"),
    ("qwen3.6-27b", "docs/using/evidence/qwen-arch-qwen35-fixed/README.md: qwen3.5-0.8b answered"),
];

/* Whether `tier` is proven on NONOS, so may be a default. */
pub fn proven(tier: &str) -> bool {
    PROVEN.iter().any(|(t, _)| *t == tier)
}

const fn same(a: &str, b: &str) -> bool {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    let mut i = 0;
    while i < a.len() && i < b.len() && a[i] == b[i] {
        i += 1;
    }
    i == a.len() && i == b.len()
}

const fn stick_proven() -> bool {
    let mut i = 0;
    while i < PROVEN.len() {
        if same(PROVEN[i].0, STICK_TIER) {
            return true;
        }
        i += 1;
    }
    false
}

/* The stick tier is a default before any other, so it must be proven. */
const _: () = assert!(stick_proven(), "the stick tier is not proven on NONOS");
