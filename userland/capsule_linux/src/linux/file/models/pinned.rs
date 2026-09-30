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
 * The models this personality vouches for, by tier: a name, its length and
 * the SHA-256 the file must have. The tables are part of the signed
 * personality, so its measurement covers every digest in them; the disk
 * only says where the bytes wait. Every file is one the Qwen team
 * publishes in its GGUF repositories on Hugging Face, each digest the one
 * its LFS pointer names. The tables sit beside this file, one a family,
 * and nothing is pinned anywhere else.
 */

use super::pinned_coder::CODER;
use super::pinned_qwen25::QWEN25;
use super::pinned_qwen25_big::QWEN25_BIG;
use super::pinned_qwen3::QWEN3;

pub struct Pinned {
    pub tier: &'static str,
    pub name: &'static [u8],
    pub bytes: u64,
    pub sha256: [u8; 32],
}

/* Every table, smallest family first. */
pub const FAMILIES: &[&[Pinned]] = &[QWEN25, QWEN25_BIG, QWEN3, CODER];

/* Every pinned file, family by family, in table order. */
pub fn all() -> impl Iterator<Item = &'static Pinned> {
    FAMILIES.iter().flat_map(|f| f.iter())
}

pub fn pin_of(name: &[u8]) -> Option<&'static Pinned> {
    all().find(|p| p.name == name)
}
