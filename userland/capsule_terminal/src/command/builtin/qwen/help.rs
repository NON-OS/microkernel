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

//! What `help qwen` says under its usage line.

/* The tiers, the models behind them, getting them, and the keys of a chat. */
pub const HELP: &[&[u8]] = &[
    b"  tiers, all offline, each model file verified by SHA-256:",
    b"    Qwen2.5: small 0.5B (default), medium 1.5B, large 3B, xlarge 7B, xxl 14B, max 32B",
    b"    Qwen3: qwen3-0.6b, qwen3-1.7b, qwen3-4b, qwen3-8b, qwen3-14b, qwen3-32b,",
    b"      qwen3-30b-a3b (MoE, 3B active, fast on a CPU)",
    b"    Coder: coder-1.5b, coder-7b, coder-14b, coder-32b",
    b"  a bigger tier needs more memory: roughly its file size plus 1-2 GB",
    b"  qwen tiers lists every tier, its download, the memory it needs, and if it is here;",
    b"  qwen get TIER... downloads tiers. Both work only on a system built with the",
    b"    signed model catalogue (the marketplace operator key), and fetch from the",
    b"    NONOS model repository when the build named one (NONOS_MODEL_MIRROR), else",
    b"    from the Qwen team's Hugging Face files. The kernel keeps a file only if",
    b"    its SHA-256 is the signed pin; Ctrl-C stops, and the next qwen get goes on",
    b"  otherwise, tools/nonos-qwen-tier.py puts a tier on the disk",
    b"  the rest of the line is the first question; it is not kept in history",
    b"  the answer streams onto the screen as it is written",
    b"  Enter sends a line; Ctrl-D on an empty line ends the chat, Ctrl-C kills it",
    b"  in the chat, /reset starts over and /exit or /bye ends it",
    b"  qwen window [tier] opens the chat in its own desktop window instead;",
    b"    the prompt comes back at once, and the conversation stays in the window",
    b"  to ask a question that starts with window, get or tiers, name a tier first",
];
