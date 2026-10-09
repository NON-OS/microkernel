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
 * What a tier needs in memory, worked out the way qwenchat works it out
 * before it loads (userland/linux_guests/cpp/qwenmem.cpp): the weights; a
 * key and a value at two bytes for every layer, KV head and head element
 * over 2048 positions, the fewest it starts with; and a margin of 64 MiB
 * plus 64 bytes for every embedding value of a 512-token batch. Each shape
 * is (layers, KV heads, head width, embedding width) from the model's
 * config.json in the Qwen team's Hugging Face repository (Qwen/<model>),
 * read on 2026-09-30.
 *
 * This table is the one source: tools/nonos_qwen_tier/shapes.py reads it to
 * write each tier's memory into the signed catalogue, the fetcher admits a
 * catalogue only when every tier's memory is this, and the store shows it
 * on a tier's card. Pure, and free of the pins: each caller brings the
 * weights, the pinned files' summed length.
 */

pub const POSITIONS: u64 = 2048;
pub const BATCH: u64 = 512;
pub const MARGIN: u64 = 64 << 20;

#[rustfmt::skip]
pub const SHAPES: &[(&str, u64, u64, u64, u64)] = &[
    ("small", 24, 2, 64, 896),             // Qwen2.5-0.5B-Instruct
    ("medium", 28, 2, 128, 1536),          // Qwen2.5-1.5B-Instruct
    ("large", 36, 2, 128, 2048),           // Qwen2.5-3B-Instruct
    ("xlarge", 28, 4, 128, 3584),          // Qwen2.5-7B-Instruct
    ("xxl", 48, 8, 128, 5120),             // Qwen2.5-14B-Instruct
    ("max", 64, 8, 128, 5120),             // Qwen2.5-32B-Instruct
    ("qwen3-0.6b", 28, 8, 128, 1024),      // Qwen3-0.6B
    ("qwen3-1.7b", 28, 8, 128, 2048),      // Qwen3-1.7B
    ("qwen3-4b", 36, 8, 128, 2560),        // Qwen3-4B
    ("qwen3-8b", 36, 8, 128, 4096),        // Qwen3-8B
    ("qwen3-14b", 40, 8, 128, 5120),       // Qwen3-14B
    ("qwen3-30b-a3b", 48, 4, 128, 2048),   // Qwen3-30B-A3B
    ("qwen3-32b", 64, 8, 128, 5120),       // Qwen3-32B
    ("coder-1.5b", 28, 2, 128, 1536),      // Qwen2.5-Coder-1.5B-Instruct
    ("coder-7b", 28, 4, 128, 3584),        // Qwen2.5-Coder-7B-Instruct
    ("coder-14b", 48, 8, 128, 5120),       // Qwen2.5-Coder-14B-Instruct
    ("coder-32b", 64, 8, 128, 5120),       // Qwen2.5-Coder-32B-Instruct
];

/* Bytes `tier` needs in memory with `weights` bytes of model files. */
pub fn memory(tier: &str, weights: u64) -> Option<u64> {
    let &(_, layers, kv_heads, head, embd) = SHAPES.iter().find(|s| s.0 == tier)?;
    let kv = layers * kv_heads * 2 * head * 2 * POSITIONS;
    Some(weights + kv + MARGIN + BATCH * embd * 64)
}

/*
 * Whether a tier fits this machine: the one rule setup's Qwen step, the
 * store's cards and the fetcher's refusal all hold a tier to, so none of
 * them offers what another refuses.
 *
 * Where the model is kept decides it. On an installed NONOS the file is on
 * the disk and only the run is held in memory: a tier fits when what it
 * needs to run (`memory`) leaves `SYSTEM` for everything else. On a live
 * boot the data volume is itself memory, so the file is held there and read
 * again into the chat program: a tier fits when the file and the run
 * together leave what the kernel keeps free while a session volume grows,
 * the larger of `SYSTEM` and a quarter of memory (src/fs/cryptoblock/ram.rs).
 */

/* Where a tier's model files are kept on this boot. */
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Room {
    /* An installed NONOS: the volume is on the disk. */
    Disk,
    /* A live boot: the volume is held in memory and gone at power off. */
    Memory,
}

/* Memory kept for the system beside a running model. */
pub const SYSTEM: u64 = 1 << 30;

/* What the kernel keeps free for the system while a session volume grows. */
pub fn session_keep(total: u64) -> u64 {
    (total / 4).max(SYSTEM)
}

/* Whether a tier of `weights` bytes of files, needing `need` to run, fits `total`. */
pub fn fits(room: Room, weights: u64, need: u64, total: u64) -> bool {
    match room {
        Room::Disk => need.saturating_add(SYSTEM) <= total,
        Room::Memory => weights.saturating_add(need) <= total.saturating_sub(session_keep(total)),
    }
}

/* Whether `tier` fits; None for a tier with no shape here. */
pub fn tier_fits(room: Room, tier: &str, weights: u64, total: u64) -> Option<bool> {
    memory(tier, weights).map(|need| fits(room, weights, need, total))
}

/*
 * The tier the release stick carries (STICK_TIER in tools/nonos_seal/
 * media.py, which the catalogue tool's tests hold to this): laid past the
 * ESP under a live plan and imported into the session's volume with no
 * network when its digest is the pin. Whether this boot's stick carries it
 * is the kernel's to say, at the import; until then it is said as the tier
 * release sticks carry, never as one known to be here.
 */
pub const STICK_TIER: &str = "qwen3-0.6b";
