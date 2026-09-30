# NONOS Operating System
# Copyright (C) 2026 NONOS Contributors
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU Affero General Public License for more details.
#
# You should have received a copy of the GNU Affero General Public License
# along with this program. If not, see <https://www.gnu.org/licenses/>.
"""What a tier needs in memory, worked out the way qwenchat works it out
before it loads (userland/linux_guests/cpp/qwenmem.cpp): the weights; a key
and a value at two bytes for every layer, KV head and head element over 2048
positions, the fewest it starts with; and a margin of 64 MiB plus 64 bytes
for every embedding value of a 512-token batch. Each shape is (layers, KV
heads, head width, embedding width) from the model's config.json in the
Qwen team's Hugging Face repository (Qwen/<model>), read on 2026-09-30.
"""

import sys

SHAPES = {
    "small": (24, 2, 64, 896),  # Qwen2.5-0.5B-Instruct
    "medium": (28, 2, 128, 1536),  # Qwen2.5-1.5B-Instruct
    "large": (36, 2, 128, 2048),  # Qwen2.5-3B-Instruct
    "xlarge": (28, 4, 128, 3584),  # Qwen2.5-7B-Instruct
    "xxl": (48, 8, 128, 5120),  # Qwen2.5-14B-Instruct
    "max": (64, 8, 128, 5120),  # Qwen2.5-32B-Instruct
    "qwen3-0.6b": (28, 8, 128, 1024),  # Qwen3-0.6B
    "qwen3-1.7b": (28, 8, 128, 2048),  # Qwen3-1.7B
    "qwen3-4b": (36, 8, 128, 2560),  # Qwen3-4B
    "qwen3-8b": (36, 8, 128, 4096),  # Qwen3-8B
    "qwen3-14b": (40, 8, 128, 5120),  # Qwen3-14B
    "qwen3-30b-a3b": (48, 4, 128, 2048),  # Qwen3-30B-A3B
    "qwen3-32b": (64, 8, 128, 5120),  # Qwen3-32B
    "coder-1.5b": (28, 2, 128, 1536),  # Qwen2.5-Coder-1.5B-Instruct
    "coder-7b": (28, 4, 128, 3584),  # Qwen2.5-Coder-7B-Instruct
    "coder-14b": (48, 8, 128, 5120),  # Qwen2.5-Coder-14B-Instruct
    "coder-32b": (64, 8, 128, 5120),  # Qwen2.5-Coder-32B-Instruct
}
POSITIONS, BATCH, MARGIN = 2048, 512, 64 << 20


def memory(tier, weights):
    """Bytes `tier` needs in memory with `weights` bytes of model files."""
    if tier not in SHAPES:
        sys.exit(f"{tier}: no shape known, so no memory need; add it to shapes.py")
    layers, kv_heads, head, embd = SHAPES[tier]
    kv = layers * kv_heads * 2 * head * 2 * POSITIONS
    return weights + kv + MARGIN + BATCH * embd * 64
